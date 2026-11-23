"""Generate the complete benchmark report from a paired run."""

import argparse
import csv
from datetime import UTC, datetime
import json
from pathlib import Path
from tempfile import NamedTemporaryFile


SIDES = ("before", "after")
STAGES = (
    "home_startup", "home_scroll", "browse_landing", "search_typing", "search_results",
    "playlist", "album", "artist", "song", "profile", "library_playlists",
)

OPERATIONS = {
    'sign_in':'Restore sign-in','own_profile':'Own profile','home':'Home','search':'Search (all four types)',
    'playlist':'Playlist detail','album':'Album detail','artist':'Artist overview',
    'browse':'Browse landing','genre':'Browse category (Pop)','made_for_you':'Made For You',
    'track':'Song metadata and play count','artist_catalogue':'Artist catalogue',
    'artist_tracks':'Artist track list','artist_profile':'Artist profile','artist_images':'Artist images',
    'album_catalogue':'Related albums and artists','album_tracks':'Album track list',
    'playlist_tracks':'Playlist track list','library_playlists':'Library playlists',
    'playlist_covers':'Playlist cover candidates','playlist_membership':'Playlist membership',
    'playlist_summary_batch':'Playlist summary refresh','saved_collection':'Saved tracks and albums',
    'saved_artists':'Saved artists',
    'pins':'Library pins','public_profile':'Public user profile','account_profile':'Own public user page',
    'track_radio':'Track radio','lyrics_id':'Lyrics by Spotify ID',
    'lyrics_text':'Lyrics by title and artist','home_revisit':'Home revisit',
}


WORKFLOWS = {
    "Session and profiles": "sign_in own_profile public_profile account_profile",
    "Home": "home",
    "Home refresh": "home_revisit",
    "Browse": "browse genre made_for_you",
    "Search": "search",
    "Playlists and library": "playlist playlist_tracks library_playlists playlist_covers playlist_membership playlist_summary_batch pins saved_artists",
    "Album and artist pages": "album album_tracks album_catalogue artist artist_catalogue artist_tracks artist_profile artist_images",
    "Song details and radio": "track track_radio",
    "Lyrics": "lyrics_id lyrics_text",
    "Saved tracks and albums": "saved_collection",
}
REWRITE_READS = {
    "playlist_context": "Playlist playback context",
    "collection_context": "Liked Songs playback context",
    "artwork": "Artwork download",
    "short_link": "Short-link lookup",
    "dealer": "Dealer session",
    "jam_invite": "Jam invitation lookup",
    "audio_read": "Audio setup and initial CDN range",
}


def api(path):
    rows = {}
    first = {}
    resolve_page = False
    requests = 0
    for line in path.read_text().splitlines():
        if line.startswith("START "):
            requests = 0
        elif line.startswith("FIRST "):
            _, name, ms = line.split()
            first[name] = int(ms)
        elif line.startswith(("HTTP ", "HTTP_RETRY")):
            requests += 1
            resolve_page |= line.endswith("/ResolvePage")
        elif line.startswith(("END ", "FAIL ")):
            result, name, ms, down, up, *items = line.split()
            rows[name] = (requests, int(ms), int(down), int(up), int(items[0]) if items else None, result)
    current = {key for keys in WORKFLOWS.values() for key in keys.split()} | REWRITE_READS.keys()
    if rows.keys() not in (OPERATIONS.keys(), current):
        raise ValueError(f"{path}: incomplete or unexpected provider workload")
    return rows, first, resolve_page


def desktop(path):
    rows = {}
    prior = (0, 0)
    elapsed = 64
    with (path / "stages.csv").open() as stream:
        for row in csv.DictReader(stream):
            stage = row["stage"]
            if stage in rows:
                raise ValueError(f"{path}: repeated checkpoint {stage}")
            traffic = (int(row["rx"]), int(row["tx"]))
            times = tuple(float(row[key]) if row[key] else None for key in ("first_visual", "settled"))
            if stage != "home_scroll" and times[0] is None:
                raise ValueError(f"{path}: no visible change at {stage}; check the navigation")
            rows[stage] = (*times, *(new - old for old, new in zip(prior, traffic)))
            prior = traffic
            if row.get("elapsed"):
                elapsed = float(row["elapsed"])
    if tuple(rows) != STAGES:
        raise ValueError(f"{path}: incomplete or unexpected desktop workload")
    return rows, prior, elapsed


def load_results(directory, side):
    """Read the stored measurements for one build."""
    path = directory / f"{side}.json"
    return json.loads(path.read_text()) if path.is_file() else {}


def save_result(directory, side, workload, root, revision, source_diff=None):
    """Validate and replace one measurement, retaining the other workload."""
    results = root / "results"
    if workload == "api":
        path = results / f"api-{side}.log"
        rows, first, resolve_page = api(path)
        result = {"rows": rows, "first_ms": first, "resolve_page": resolve_page}
    else:
        folder = results / f"gui-{side}"
        path = folder / "stages.csv"
        rows, totals, elapsed = desktop(folder)
        result = {"rows": rows, "totals": totals, "elapsed": elapsed,
                  "executable_bytes": (root / f"sonora-{side}").stat().st_size}
    result["revision"] = revision
    if source_diff:
        result["source_diff"] = source_diff
    result["date"] = datetime.fromtimestamp(path.stat().st_mtime, UTC).isoformat(timespec="seconds")
    snapshot = load_results(directory, side)
    snapshot[workload] = result
    directory.mkdir(parents=True, exist_ok=True)
    with NamedTemporaryFile(mode="w", dir=directory, delete=False) as output:
        temporary = Path(output.name)
        json.dump(snapshot, output, indent=2)
        output.write("\n")
    try:
        temporary.replace(directory / f"{side}.json")
    finally:
        temporary.unlink(missing_ok=True)


def compared_revisions(before, after):
    def source(run):
        revision = f"`{run['revision'][:8]}`"
        if diff := run.get("source_diff"):
            revision += f" with local source changes (`{diff[:8]}`)"
        return f"{revision} at {run['date']}"

    return f"Upstream {source(before)} → rewrite {source(after)}"



def table(headers, rows):
    def line(cells):
        return "| " + " | ".join(map(str, cells)) + " |"

    return "\n".join([
        line(headers),
        line(["---", *["---:"] * (len(headers) - 1)]),
        *(line(row) for row in rows),
    ])


def marker(before, after, minimum=0):
    if before is None or after is None or before == after or abs(after - before) < minimum:
        return ""
    if after > before:
        return " 🟥"
    return " 🟩" if after <= before * 0.75 else ""


def number(value, scale=1, decimals=0):
    return "—" if value is None else f"{value / scale:,.{decimals}f}"


def pair(before, after, scale=1, decimals=0, minimum=0):
    return f"{number(before, scale, decimals)} → {number(after, scale, decimals)}{marker(before, after, minimum)}"


def summary(name, before, after, scale=1, decimals=0, unit=""):
    change = f"{(after / before - 1):+.1%}".replace("-", "−") if before else "—"
    return (name, number(before, scale, decimals) + unit, number(after, scale, decimals) + unit,
            change + marker(before, after))


def section(title, revisions, description, body):
    return f"## {title}\n\n{revisions}\n\n{description}\n\n{body}"


def generate(directory):
    upstream, candidate = (load_results(directory, side) for side in SIDES)
    overview = []
    sections = []

    if "desktop" in upstream and "desktop" in candidate:
        old_desktop, new_desktop = upstream["desktop"], candidate["desktop"]
        before, after = old_desktop["rows"], new_desktop["rows"]
        old_totals, new_totals = old_desktop["totals"], new_desktop["totals"]
        old_time, new_time = (run.get("elapsed", 64) for run in (old_desktop, new_desktop))
        overview.append(summary("Release executable", old_desktop["executable_bytes"],
                                new_desktop["executable_bytes"], 1e6, 2, " MB"))
        overview.extend([
            summary("Desktop download", old_totals[0], new_totals[0], 1e6, 2, " MB"),
            summary("Desktop upload", old_totals[1], new_totals[1], 1e3, 1, " KB"),
        ])
        sections.append(section(
            "Desktop browsing",
            compared_revisions(old_desktop, new_desktop),
            f"Fresh-cache browsing, including artwork and background traffic. "
            f"Pass duration was {old_time:g} → {new_time:g} seconds.",
            table(
                ["Stage", "First change (s)", "Settled (s)", "Download (KiB)", "Upload (KiB)"],
                [(name.replace("_", " ").capitalize(),
                  *(pair(before[name][i], after[name][i], 1, 2, 0.24) for i in (0, 1)),
                  *(pair(before[name][i], after[name][i], 1024, 1) for i in (2, 3)))
                 for name in STAGES],
            ),
        ))

    if "api" in upstream and "api" in candidate:
        old_api, new_api = upstream["api"], candidate["api"]
        old, new = old_api["rows"], new_api["rows"]
        workflows = {
            label: [key for key in keys.split() if key in old and key in new
                    and old[key][-1] == new[key][-1] == "END"]
            for label, keys in WORKFLOWS.items()
        }
        completed = [key for keys in workflows.values() for key in keys]
        metrics = (
            ("Provider HTTP calls", 1, 0, ""),
            ("Provider time", 1000, 2, " s"),
            ("Provider download", 1024, 1, " KiB"),
            ("Provider upload", 1024, 1, " KiB"),
        )
        for i, (name, scale, precision, unit) in enumerate(metrics):
            totals = [sum(rows[key][i] for key in completed) for rows in (old, new)]
            overview.append(summary(name, *totals, scale, precision, unit))

        headers = ["Workflow", "HTTP calls", "Time (ms)", "Download (KiB)", "Upload (KiB)"]
        groups = [
            (label, *(pair(*(sum(rows[key][i] for key in keys) for rows in (old, new)),
                          scale, precision)
                      for i, (scale, precision) in enumerate(((1, 0), (1, 0), (1024, 1), (1024, 1)))))
            for label, keys in workflows.items() if keys
        ]
        sections.append(section(
            "Spotify API",
            compared_revisions(old_api, new_api),
            f"Totals cover {len(completed)} matched reads. Time sums provider calls and excludes rendering. "
            "Home uses the app’s provider feed, including concurrent shelf reads. Home refresh explicitly rereads that feed.",
            table(headers, groups),
        ))
        first = new_api.get("first_ms", {})
        if "home" in first and "saved_collection" in first:
            sections.append(f"First provider result on the rewrite was {first['home']:,} ms for Home "
                            f"and {first['saved_collection']:,} ms for Liked Songs.")
        for side, rows in (("Upstream", old), ("Rewrite", new)):
            failed = [OPERATIONS.get(key, REWRITE_READS.get(key, key))
                      for key, row in rows.items() if row[-1] != "END"]
            if failed:
                sections.append(f"{side} reads that failed: " + ", ".join(failed) + ".")
    doc = [
        "<!-- Generated by benchmark/run.py. -->",
        "# Spotify benchmark vs upstream main",
        "## Summary",
        "Values read upstream → rewrite, with 🟩 for reductions of at least 25% and 🟥 for increases.",
        table(["Metric", "Upstream", "Rewrite", "Change"], overview) if overview
        else "A comparison appears once both builds have results for a workload.",
        *sections,
        "## Reproduce",
        "Enter the Sonora build environment with Python 3.12 or later and a C compiler. "
        "For desktop runs, use Niri with XWayland and the X11 and XTest libraries.",
        r"""```sh
python3 crates/music/src/spotify/benchmark/run.py target/spotify-api-run \
  --side both --workload api \
  --before-account /path/to/upstream/spotify --after-account /path/to/candidate/spotify
```""",
        "Choose `--side before`, `after`, or `both` and `--workload api`, `desktop`, or `both`. "
        "Point `--before-account` at the folder containing `credentials.json` for upstream and "
        "`--after-account` at the folder containing `spotify-oauth.json` for the rewrite. "
        "Use the same machine and Spotify account for both builds.",
        "Add `--working-tree` to measure local changes to the rewrite.",
        "Find the report in `results/benchmark.md` under the output directory. "
        "Use `--report` to save it somewhere else too. Keep the output directory private.",
    ]
    return "\n\n".join(doc) + "\n"


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("directory", nargs="?", type=Path, default=Path(__file__).resolve().parent)
    args = parser.parse_args()
    print(generate(args.directory), end="")
