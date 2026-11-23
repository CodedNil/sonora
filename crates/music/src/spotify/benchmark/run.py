#!/usr/bin/env python3
"""Build, measure and report a paired Sonora Spotify benchmark on Linux/Niri."""

import argparse
import ctypes
import hashlib
import mmap
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile

from report import generate, save_result


TOOLS = Path(__file__).resolve().parent
REPO = TOOLS.parents[4]
CREDENTIALS = {
    "before": "credentials.json",
    "after": "spotify-oauth.json",
}


def archive(revision, destination):
    """Extract tracked files from one Git revision without switching branches."""
    destination.mkdir(parents=True)
    with subprocess.Popen(
        ["git", "archive", revision], cwd=REPO, stdout=subprocess.PIPE
    ) as git:
        with tarfile.open(fileobj=git.stdout, mode="r|") as tar:
            tar.extractall(destination, filter="data")
        if git.wait():
            raise RuntimeError(f"git archive failed for {revision}")


def instrument(source, side):
    """Add the API example and request counters to an isolated source tree."""
    example = source / "crates/music/examples/spotify_benchmark.rs"
    example.parent.mkdir(parents=True, exist_ok=True)
    if side == "before":
        shutil.copyfile(TOOLS / "provider.rs", example)
        subprocess.run(["git", "apply", TOOLS / "before.patch"], cwd=source, check=True)
        return
    shutil.copyfile(TOOLS / "provider_after.rs", example)

    def patch(path, old, new):
        path = source / path
        content = path.read_text()
        if content.count(old) != 1:
            raise RuntimeError(f"update benchmark instrumentation for {path}")
        path.write_text(content.replace(old, new))

    patch("crates/spotifyapis/src/http.rs", "    build(client).send().await\n}", """    let request = build(client).build()?;
    println!("HTTP {} {}{}", request.method(), request.url().host_str().unwrap_or_default(), request.url().path());
    client.execute(request).await
}""")
    http = source / "crates/spotifyapis/src/http.rs"
    retry = "response_bytes(build(client).send().await?).await"
    if retry in http.read_text():
        patch("crates/spotifyapis/src/http.rs", retry, """let request = build(client).build()?;
            println!("HTTP_RETRY {} {}{}", request.method(), request.url().host_str().unwrap_or_default(), request.url().path());
            response_bytes(client.execute(request).await?).await""")
    patch("crates/music/src/spotify/client.rs", "impl SpotifyClient {", """impl SpotifyClient {
    /// Expose the current session to the isolated benchmark.
    pub fn benchmark_session(&self) -> (&std::sync::Arc<spotifyapis::Session>, &str) {
        (self.session.api(), self.session.device_id())
    }
""")
    patch("crates/music/src/link.rs", "    let mut response = client", """    println!("HTTP short link");
    let mut response = client""")
    patch("crates/music/src/link.rs", "        attempt.follow()", """        println!("HTTP short link redirect");
        attempt.follow()""")


def snapshot(destination):
    """Copy the working tree and fingerprint its contents."""
    digest = hashlib.sha256()
    paths = subprocess.check_output(
        ["git", "ls-files", "--cached", "--others", "--exclude-standard", "-z"], cwd=REPO
    ).split(b"\0")
    for name in sorted(set(filter(None, paths))):
        relative = Path(os.fsdecode(name))
        source = REPO / relative
        if not source.is_file():
            continue
        output = destination / relative
        output.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(source, output)
        digest.update(name)
        digest.update(hashlib.sha256(output.read_bytes()).digest())
    return digest.hexdigest()


def seed(data, source, side):
    """Copy only the sign-in material and device ID into a private data directory."""
    account = data / "sonora/accounts/spotify"
    account.mkdir(parents=True)
    required = CREDENTIALS[side]
    shutil.copyfile(source / required, account / required)
    if (source / "device-id").is_file():
        shutil.copyfile(source / "device-id", account / "device-id")
    return account


def build(source, target, arguments, executable, destination):
    env = os.environ | {"CARGO_TARGET_DIR": str(target)}
    subprocess.run(["cargo", "build", "--locked", *arguments], cwd=source, env=env, check=True)
    shutil.copyfile(target / executable, destination)
    destination.chmod(0o700)


def verify_english_bundle(source, executable):
    """Catch stale embedded locale files when reusing a Cargo build cache."""
    with executable.open("rb") as binary, mmap.mmap(
        binary.fileno(), 0, access=mmap.ACCESS_READ
    ) as payload:
        for resource in sorted((source / "assets/i18n/en-US").glob("*.ftl")):
            if payload.find(resource.read_bytes()) == -1:
                raise RuntimeError(
                    f"{executable.name} does not embed the expected {resource.name}. "
                    "Run again with a fresh Cargo target directory."
                )


def provider(root, side, credentials):
    isolated = root / "provider" / side
    paths = {kind: isolated / kind.lower() for kind in ("DATA", "CACHE", "STATE", "CONFIG")}
    for path in paths.values():
        path.mkdir(parents=True)
    account = seed(paths["DATA"], credentials, side)
    env = os.environ | {f"XDG_{kind}_HOME": str(path) for kind, path in paths.items()}
    env |= {
        "LD_PRELOAD": str(root / "netcount.so"),
        "SONORA_PROFILE_NETWORK": str(root / "results" / f"api-{side}.dat"),
    }
    log = root / "results" / f"api-{side}.log"
    with log.open("w") as stream:
        status = subprocess.run(
            [root / f"api-probe-{side}"], env=env, stdout=stream, stderr=subprocess.STDOUT
        ).returncode
    if status:
        raise RuntimeError(f"{side} provider pass failed. See {log}")
    return account


def desktop(root, side, credentials):
    run = root / "results" / f"gui-{side}"
    account = seed(run / "data", credentials, side)
    log = root / "results" / f"gui-{side}-events.log"
    with log.open("w") as stream:
        status = subprocess.run(
            [sys.executable, TOOLS / "desktop.py", root, side],
            stdout=stream,
            stderr=subprocess.STDOUT,
        ).returncode
    if status:
        raise RuntimeError(f"{side} desktop pass failed. See {log}")
    return account


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("output", type=Path, help="new directory for private benchmark output")
    parser.add_argument("--before", default="origin/main", help="upstream Git revision")
    parser.add_argument("--after", default="HEAD", help="candidate Git revision")
    parser.add_argument("--side", choices=("before", "after", "both"), default="both",
                        help="which build measurements to refresh")
    workload = parser.add_mutually_exclusive_group()
    workload.add_argument("--workload", choices=("api", "desktop", "both"), default="both",
                          help="which workload measurements to refresh")
    workload.add_argument("--api-only", dest="workload", action="store_const", const="api",
                          help=argparse.SUPPRESS)
    parser.add_argument("--data-dir", type=Path, default=TOOLS, help="directory for saved JSON results")
    parser.add_argument("--report", type=Path, help="also write the generated Markdown here")
    parser.add_argument("--before-account", type=Path, help="upstream Spotify account directory")
    parser.add_argument("--after-account", type=Path, help="rewrite Spotify account directory")
    parser.add_argument("--target-dir", type=Path, help="reuse an existing Cargo target cache")
    parser.add_argument(
        "--prepare-only", action="store_true", help="extract revisions and verify patches offline"
    )
    parser.add_argument("--working-tree", action="store_true", help="measure local candidate source")
    args = parser.parse_args()
    sides = tuple(CREDENTIALS) if args.side == "both" else (args.side,)
    workloads = ("api", "desktop") if args.workload == "both" else (args.workload,)
    data = args.data_dir.resolve()
    revisions = {
        side: subprocess.check_output(["git", "rev-parse", "--verify", f"{getattr(args, side)}^{{commit}}"],
                                      cwd=REPO, text=True).strip()
        for side in sides
    }
    accounts = {"before": args.before_account, "after": args.after_account}
    if not args.prepare_only:
        for side in sides:
            path = accounts[side]
            if path is None or not (path / CREDENTIALS[side]).is_file():
                parser.error(f"--{side}-account must contain {CREDENTIALS[side]}")
        if "desktop" in workloads:
            if not (os.environ.get("DISPLAY") and shutil.which("niri")):
                parser.error("desktop measurements require Niri and XWayland")
            for library in ("libX11.so.6", "libXtst.so.6"):
                try:
                    ctypes.CDLL(library)
                except OSError:
                    parser.error(f"the desktop runner cannot load {library}")

    os.umask(0o077)
    root = args.output.resolve()
    root.mkdir(mode=0o700, parents=True)
    results = root / "results"
    results.mkdir()
    target = args.target_dir.resolve() if args.target_dir else root / "target"
    source_diffs = {}
    for side, revision in revisions.items():
        source = root / "source" / side
        print(f"Preparing {side} ({revision[:8]})", flush=True)
        if side == "after" and args.working_tree:
            source_diffs[side] = snapshot(source)
        else:
            archive(revision, source)
        if "desktop" in workloads and not args.prepare_only:
            for resource in (source / "assets/i18n").glob("*/*.ftl"):
                resource.touch()
            exe = root / f"sonora-{side}"
            build(source, target / side, ["--release", "-p", "sonora"],
                  "release/sonora", exe)
            verify_english_bundle(source, exe)
        if "api" in workloads:
            instrument(source, side)
            if not args.prepare_only:
                build(source, target / side, ["-p", "music", "--example", "spotify_benchmark"],
                      "debug/examples/spotify_benchmark", root / f"api-probe-{side}")

    if args.prepare_only:
        return

    subprocess.run(
        [os.environ.get("CC", "cc"), "-O2", "-shared", "-fPIC",
         TOOLS / "netcount.c", "-ldl", "-o", root / "netcount.so"],
        check=True,
    )
    for workload, measure in (("api", provider), ("desktop", desktop)):
        if workload in workloads:
            for side, revision in revisions.items():
                print(f"Measuring {side} {workload}", flush=True)
                accounts[side] = measure(root, side, accounts[side])
                save_result(data, side, workload, root, revision, source_diffs.get(side))

    report = generate(data)
    (results / "benchmark.md").write_text(report)
    if args.report:
        args.report.write_text(report)
    print(f"Report: {results / 'benchmark.md'}")
    if "after" in sides:
        print(f"Latest rewrite sign-in, if rotated: {accounts['after']}")


if __name__ == "__main__":
    main()
