"""Store benchmark results and render them as BENCHMARKS.md."""

import csv
from datetime import UTC, datetime
import json
from pathlib import Path


SIDES = ("before", "after")
STAGES = (
    "home_startup", "home_scroll", "browse_landing", "search_preview", "search_results",
    "playlist", "album", "artist", "song", "profile", "library_playlists",
)
# Provider reads grouped by what a listener does. Reads only one build makes are left out.
WORKFLOWS = {
    "Session and profiles": "sign_in own_profile public_profile account_profile",
    "Home": "home",
    "Home refresh": "home_revisit",
    "Browse page": "browse",
    "Browse categories": "genre made_for_you",
    "Search": "search",
    "Playlists and library": "playlist playlist_tracks library_playlists playlist_covers "
                             "playlist_membership playlist_summary_batch pins saved_artists",
    "Album and artist pages": "album album_tracks album_catalogue artist artist_catalogue "
                              "artist_tracks artist_profile artist_images",
    "Song details and radio": "track track_radio",
    "Lyrics": "lyrics_id lyrics_text",
    "Saved tracks and albums": "saved_collection",
}
READS = [key for keys in WORKFLOWS.values() for key in keys.split()]


def api(path):
    """Per read: HTTP calls, milliseconds, bytes down, bytes up, outcome; and first results."""
    rows, first, calls = {}, {}, 0
    for line in path.read_text().splitlines():
        kind, *fields = line.split()
        if kind == "START":
            calls = 0
        elif kind in ("HTTP", "HTTP_RETRY"):
            calls += 1
        elif kind == "FIRST":
            first[fields[0]] = int(fields[1])
        elif kind in ("END", "FAIL"):
            name, ms, down, up = fields[:4]
            rows[name] = (calls, int(ms), int(down), int(up), kind)
    if missing := set(READS) - rows.keys():
        raise ValueError(f"{path}: missing reads {sorted(missing)}")
    return {"rows": rows, "first_ms": first}


def desktop(folder):
    """Per stage: seconds until settled, and bytes down and up during the stage."""
    rows, prior = {}, (0, 0)
    with (folder / "stages.csv").open() as stream:
        for row in csv.DictReader(stream):
            traffic = (int(row["rx"]), int(row["tx"]))
            settled = float(row["settled"]) if row["settled"] else None
            rows[row["stage"]] = (settled, traffic[0] - prior[0], traffic[1] - prior[1])
            prior = traffic
    if tuple(rows) != STAGES:
        raise ValueError(f"{folder}: unexpected stages {list(rows)}")
    return {"rows": rows, "totals": prior}


def save_result(directory, side, workload, root, revision, source_diff=None):
    """Replace one workload's measurement for one build, keeping the other workload."""
    results = root / "results"
    if workload == "api":
        log = results / f"api-{side}.log"
        result = api(log)
    else:
        log = results / f"gui-{side}" / "stages.csv"
        result = desktop(log.parent)
        result["executable_bytes"] = (root / f"sonora-{side}").stat().st_size
    result |= {"revision": revision, "source_diff": source_diff,
               "date": datetime.fromtimestamp(log.stat().st_mtime, UTC).isoformat(timespec="seconds")}
    path = directory / f"{side}.json"
    stored = json.loads(path.read_text()) if path.is_file() else {}
    stored[workload] = result
    path.write_text(json.dumps(stored, indent=2) + "\n")


def table(headers, rows):
    line = lambda cells: "| " + " | ".join(map(str, cells)) + " |"
    return "\n".join([line(headers), line(["---", *["---:"] * (len(headers) - 1)]), *map(line, rows)])


def marker(before, after, minimum=0):
    """Green or red for a change of at least 10% that is also larger than `minimum`."""
    if before is None or after is None or abs(after - before) <= minimum:
        return ""
    return " 🟥" if after >= before * 1.1 else " 🟩" if after <= before * 0.9 else ""


def number(value, scale=1, decimals=0):
    return "—" if value is None else f"{value / scale:,.{decimals}f}"


def pair(before, after, scale=1, decimals=0, minimum=0):
    return f"{number(before, scale, decimals)} → {number(after, scale, decimals)}{marker(before, after, minimum)}"


def summary(name, before, after, scale, decimals, unit):
    change = f"{after / before - 1:+.1%}".replace("-", "−")
    return name, number(before, scale, decimals) + unit, number(after, scale, decimals) + unit, change + marker(before, after)


def revisions(before, after):
    def source(run):
        local = f" with local source changes (`{run['source_diff'][:8]}`)" if run.get("source_diff") else ""
        return f"`{run['revision'][:8]}`{local} at {run['date']}"
    return f"Upstream {source(before)} → rewrite {source(after)}"


def generate(directory):
    old, new = ({} if not (path := directory / f"{side}.json").is_file() else json.loads(path.read_text())
                for side in SIDES)
    overview, sections = [], []

    if "desktop" in old and "desktop" in new:
        before, after = old["desktop"], new["desktop"]
        overview += [
            summary("Release executable", before["executable_bytes"], after["executable_bytes"], 1e6, 2, " MB"),
            summary("Desktop download", before["totals"][0], after["totals"][0], 1e6, 2, " MB"),
            summary("Desktop upload", before["totals"][1], after["totals"][1], 1e3, 1, " KB"),
        ]
        rows = [(stage.replace("_", " ").capitalize(),
                 pair(before["rows"][stage][0], after["rows"][stage][0], 1, 2, 0.1),
                 pair(before["rows"][stage][1], after["rows"][stage][1], 1024, 1),
                 pair(before["rows"][stage][2], after["rows"][stage][2], 1024, 1))
                for stage in STAGES]
        sections.append("\n\n".join([
            "## Desktop browsing", revisions(before, after),
            "Each build starts with an empty cache. Settled is the time from the action to the last "
            "change on screen, followed by a second with no change on screen and no network traffic; "
            "— means it never settled within the stage's budget. The screen is sampled about every "
            "50 ms. Search preview is timed from the last keystroke of the query, after the rest of "
            "it was typed and settled. Traffic counts everything the app sent and received during "
            "the stage, artwork included.",
            table(["Stage", "Settled (s)", "Download (KiB)", "Upload (KiB)"], rows),
        ]))

    if "api" in old and "api" in new:
        before, after = old["api"]["rows"], new["api"]["rows"]
        done = {label: [key for key in keys.split() if before[key][4] == after[key][4] == "END"]
                for label, keys in WORKFLOWS.items()}
        reads = [key for keys in done.values() for key in keys]
        for i, (name, scale, decimals, unit) in enumerate((
                ("Provider HTTP calls", 1, 0, ""), ("Provider time", 1000, 2, " s"),
                ("Provider download", 1024, 1, " KiB"), ("Provider upload", 1024, 1, " KiB"))):
            overview.append(summary(name, *(sum(rows[key][i] for key in reads) for rows in (before, after)),
                                    scale, decimals, unit))
        rows = [(label, *(pair(*(sum(rows[key][i] for key in keys) for rows in (before, after)), scale, decimals)
                          for i, (scale, decimals) in enumerate(((1, 0), (1, 0), (1024, 1), (1024, 1)))))
                for label, keys in done.items() if keys]
        first = new["api"]["first_ms"]
        notes = [f"Each read calls the provider the way its build's app does. Time sums the reads in a "
                 f"row and excludes rendering; totals cover {len(reads)} reads both builds completed. "
                 "Browse page is opening the Browse tab. Browse categories opens Pop and Made For You "
                 "from their Browse cards; the rewrite shows a preview before the rest arrives, upstream "
                 "shows the page once."]
        if all(key in first for key in ("home", "genre", "made_for_you")):
            notes.append(f"First content on the rewrite arrived after {first['home']:,} ms for Home, "
                         f"{first['genre']:,} ms for Pop and {first['made_for_you']:,} ms for Made For You.")
        failed = [f"{side} could not read {key}" for side, rows in (("Upstream", before), ("The rewrite", after))
                  for key in READS if rows[key][4] != "END"]
        sections.append("\n\n".join([
            "## Spotify API", revisions(old["api"], new["api"]), *notes,
            table(["Workflow", "HTTP calls", "Time (ms)", "Download (KiB)", "Upload (KiB)"], rows),
            *([". ".join(failed) + "."] if failed else []),
        ]))

    return "\n\n".join([
        "<!-- Generated by benchmark/run.py. -->",
        "# Spotify benchmark vs upstream main",
        "## Summary",
        "Values read upstream → rewrite, with 🟩 for reductions and 🟥 for increases of at least 10%.",
        table(["Metric", "Upstream", "Rewrite", "Change"], overview) if overview
        else "A comparison appears once both builds have results for a workload.",
        *sections,
        "## Reproduce",
        "Use Python 3.12 or later and a C compiler, for example under `nix shell nixpkgs#python3`. "
        "Desktop runs need Niri with XWayland and the X11 and XTest libraries; on NixOS, add "
        "libXtst to the dev shell's library path rather than replacing it, as in "
        "`LD_LIBRARY_PATH=$(nix build --no-link --print-out-paths nixpkgs#xorg.libXtst)/lib:$LD_LIBRARY_PATH`.",
        "```sh\npython3 crates/spotify/src/benchmark/run.py target/spotify-bench \\\n"
        "  --workload api --before-account /path/to/upstream/spotify --after-account /path/to/rewrite/spotify\n```",
        "`--side` picks `before`, `after` or `both`, and `--workload` picks `api`, `desktop` or `both`. "
        "The upstream account folder holds `credentials.json` and the rewrite's holds "
        "`spotify-oauth.json`; use the same machine and account for both. `--working-tree` measures "
        "uncommitted changes to the rewrite. The report is written to `results/benchmark.md` under "
        "the output folder, which should be kept private.",
    ]) + "\n"


if __name__ == "__main__":
    print(generate(Path(__file__).resolve().parent), end="")
