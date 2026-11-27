# Spotify playback capture

See [the protocol notes](SPOTIFY_PLAYBACK_PROTOCOL.md) for current implementation status, open questions, and the Linux capture findings.

- `raw_flows.mitm` contains full payloads and can be opened offline with `mitmweb -r raw_flows.mitm`.
- `events.jsonl` is the flattened capture used by the analysis scripts. Large response bodies, including audio, are only in the raw capture.
- `analysis/` contains decoded excerpts. `scripts/` contains the capture and analysis helpers. `mitmdump.log` records the original proxy run.
- `conf/` holds the original proxy certificate material.

The captures and decoded excerpts contain account credentials and identifiers. The proxy configuration contains a CA private key. Keep these files local.
