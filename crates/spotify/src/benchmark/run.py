#!/usr/bin/env python3
"""Build upstream and the rewrite, measure them on this machine and write BENCHMARKS.md."""

import argparse
import ctypes
import hashlib
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile

from report import generate, save_result

TOOLS = Path(__file__).resolve().parent
REPO = TOOLS.parents[3]
CREDENTIALS = {"before": "credentials.json", "after": "spotify-oauth.json"}


def extract(revision, destination):
    """Copy one revision's tracked files without switching branches."""
    destination.mkdir(parents=True)
    with subprocess.Popen(["git", "archive", revision], cwd=REPO, stdout=subprocess.PIPE) as git:
        with tarfile.open(fileobj=git.stdout, mode="r|") as tar:
            tar.extractall(destination, filter="data")


def snapshot(destination):
    """Copy the working tree, returning a fingerprint of what was copied."""
    digest = hashlib.sha256()
    names = subprocess.check_output(
        ["git", "ls-files", "--cached", "--others", "--exclude-standard", "-z"], cwd=REPO
    ).split(b"\0")
    for name in sorted(set(filter(None, names))):
        source = REPO / os.fsdecode(name)
        if source.is_file():
            target = destination / os.fsdecode(name)
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(source, target)
            digest.update(name + hashlib.sha256(target.read_bytes()).digest())
    return digest.hexdigest()


def patch(source, path, old, new):
    path = source / path
    content = path.read_text()
    if content.count(old) != 1:
        raise RuntimeError(f"the benchmark instrumentation no longer fits {path}")
    path.write_text(content.replace(old, new))


def instrument(source, side):
    """Add the provider probe and print a line for every HTTP request it makes."""
    probe = source / "crates/spotify/examples/spotify_benchmark.rs"
    probe.parent.mkdir(parents=True, exist_ok=True)
    if side == "before":
        shutil.copyfile(TOOLS / "provider.rs", probe)
        subprocess.run(["git", "apply", TOOLS / "before.patch"], cwd=source, check=True)
        return
    shutil.copyfile(TOOLS / "provider_after.rs", probe)
    line = 'println!("{} {} {}{}", "{kind}", request.method(), request.url().host_str().unwrap_or_default(), request.url().path());'
    patch(source, "crates/spotifyapis/src/http.rs", "    build(client).send().await\n}",
          "    let request = build(client).build()?;\n    "
          + line.replace("{kind}", "HTTP") + "\n    client.execute(request).await\n}")
    patch(source, "crates/spotifyapis/src/http.rs", "response_bytes(build(client).send().await?).await",
          "let request = build(client).build()?;\n            "
          + line.replace("{kind}", "HTTP_RETRY") + "\n            response_bytes(client.execute(request).await?).await")
    patch(source, "crates/spotify/src/client.rs", "impl SpotifyClient {",
          "impl SpotifyClient {\n    pub fn benchmark_session(&self) -> (&std::sync::Arc<spotifyapis::Session>, &str) {\n"
          "        (self.session.api(), self.session.device_id())\n    }\n")
    patch(source, "crates/music/src/link.rs", "    let mut response = client",
          '    println!("HTTP short link");\n    let mut response = client')
    patch(source, "crates/music/src/link.rs", "        attempt.follow()",
          '        println!("HTTP short link redirect");\n        attempt.follow()')


def seed(data, account, side):
    """Copy only the sign-in and the device id into a private data folder."""
    folder = data / "sonora/accounts/spotify"
    folder.mkdir(parents=True)
    for name in (CREDENTIALS[side], "device-id"):
        if (account / name).is_file():
            shutil.copyfile(account / name, folder / name)
    return folder


def build(source, target, arguments, built, destination):
    env = os.environ | {"CARGO_TARGET_DIR": str(target)}
    subprocess.run(["cargo", "build", "--locked", *arguments], cwd=source, env=env, check=True)
    shutil.copyfile(target / built, destination)
    destination.chmod(0o700)


def measure_api(root, side, account):
    home = root / "provider" / side
    env = os.environ | {f"XDG_{kind}_HOME": str(home / kind.lower()) for kind in ("DATA", "CACHE", "STATE", "CONFIG")}
    account = seed(home / "data", account, side)
    env |= {"LD_PRELOAD": str(root / "netcount.so"), "SONORA_PROFILE_NETWORK": str(root / "results" / f"api-{side}.dat")}
    log = root / "results" / f"api-{side}.log"
    with log.open("w") as output:
        if subprocess.run([root / f"api-probe-{side}"], env=env, stdout=output, stderr=subprocess.STDOUT).returncode:
            raise RuntimeError(f"the {side} provider pass failed, see {log}")
    return account


def measure_desktop(root, side, account):
    account = seed(root / "results" / f"gui-{side}" / "data", account, side)
    log = root / "results" / f"gui-{side}.log"
    with log.open("w") as output:
        if subprocess.run([sys.executable, TOOLS / "desktop.py", root, side], stdout=output,
                          stderr=subprocess.STDOUT).returncode:
            raise RuntimeError(f"the {side} desktop pass failed, see {log}")
    return account


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("output", type=Path, help="a new private folder for builds and logs")
    parser.add_argument("--before", default="origin/main", help="the upstream revision")
    parser.add_argument("--after", default="HEAD", help="the rewrite revision")
    parser.add_argument("--side", choices=("before", "after", "both"), default="both")
    parser.add_argument("--workload", choices=("api", "desktop", "both"), default="both")
    parser.add_argument("--before-account", type=Path, help="upstream's Spotify account folder")
    parser.add_argument("--after-account", type=Path, help="the rewrite's Spotify account folder")
    parser.add_argument("--target-dir", type=Path, help="a Cargo target folder to reuse")
    parser.add_argument("--working-tree", action="store_true", help="measure uncommitted changes")
    args = parser.parse_args()
    sides = list(CREDENTIALS) if args.side == "both" else [args.side]
    workloads = ["api", "desktop"] if args.workload == "both" else [args.workload]
    accounts = {"before": args.before_account, "after": args.after_account}
    for side in sides:
        if not accounts[side] or not (accounts[side] / CREDENTIALS[side]).is_file():
            parser.error(f"--{side}-account must hold {CREDENTIALS[side]}")
    if "desktop" in workloads:
        if not (os.environ.get("DISPLAY") and shutil.which("niri")):
            parser.error("desktop runs need Niri with XWayland")
        for library in ("libX11.so.6", "libXtst.so.6"):
            try:
                ctypes.CDLL(library)
            except OSError:
                parser.error(f"desktop runs need {library} on the library path")

    os.umask(0o077)
    root = args.output.resolve()
    (root / "results").mkdir(parents=True)
    target = args.target_dir.resolve() if args.target_dir else root / "target"
    revisions, diffs = {}, {}
    for side in sides:
        revisions[side] = subprocess.check_output(
            ["git", "rev-parse", "--verify", getattr(args, side) + "^{commit}"], cwd=REPO, text=True).strip()
        source = root / "source" / side
        print(f"Preparing {side} ({revisions[side][:8]})", flush=True)
        if side == "after" and args.working_tree:
            diffs[side] = snapshot(source)
        else:
            extract(revisions[side], source)
        if "desktop" in workloads:
            # a cached build can keep stale embedded Fluent files, so make them rebuild
            for resource in (source / "assets/i18n").glob("*/*.ftl"):
                resource.touch()
            build(source, target / side, ["--release", "-p", "sonora"], "release/sonora", root / f"sonora-{side}")
        if "api" in workloads:
            instrument(source, side)
            build(source, target / side, ["-p", "spotify", "--example", "spotify_benchmark"],
                  "debug/examples/spotify_benchmark", root / f"api-probe-{side}")

    subprocess.run([os.environ.get("CC", "cc"), "-O2", "-shared", "-fPIC", TOOLS / "netcount.c", "-ldl",
                    "-o", root / "netcount.so"], check=True)
    for workload in workloads:
        measure = measure_api if workload == "api" else measure_desktop
        for side in sides:
            print(f"Measuring {side} {workload}", flush=True)
            accounts[side] = measure(root, side, accounts[side])
            save_result(TOOLS, side, workload, root, revisions[side], diffs.get(side))

    report = generate(TOOLS)
    (root / "results" / "benchmark.md").write_text(report)
    (TOOLS.parent / "BENCHMARKS.md").write_text(report)
    print(f"Report: {TOOLS.parent / 'BENCHMARKS.md'}")
    if "after" in sides:
        print(f"The rewrite's sign-in after the run, which Spotify may have rotated: {accounts['after']}")


if __name__ == "__main__":
    main()
