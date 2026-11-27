"""Drive one Sonora build through a fresh-cache browsing pass and write its stages.csv.

Usage: desktop.py OUTPUT before|after. Runs under Niri with XWayland: the app is forced onto X11 so
that keys, clicks and screen reads go through XTest and XGetImage.
"""

import ctypes as C
import csv
import json
import os
from pathlib import Path
import struct
import subprocess as sp
import sys
import time

QUIET = 1.0  # seconds with no change on screen and no traffic that count as settled
SAMPLE = 0.05  # seconds between screen reads
QUERY = "rick astley"

root, side = Path(sys.argv[1]), sys.argv[2]
run = root / "results" / f"gui-{side}"
exe = root / f"sonora-{side}"
(run / "config/sonora").mkdir(parents=True)
(run / "config/sonora/settings.json").write_text(json.dumps({
    "startup": "home", "provider": "spotify", "tray_icon": False, "close_to_tray": False,
    "spotify_connect": False, "lyrics_providers": [], "keybinds": {"open-search": ["f8"]},
    # only Library > Playlists is left in the sidebar, so that click cannot land on a pin
    "hidden_nav": ["home", "search", "history", "settings", "songs", "albums", "artists", "pins"],
}))
env = {key: value for key, value in os.environ.items() if key not in ("WAYLAND_DISPLAY", "WAYLAND_SOCKET")}
env |= {f"XDG_{kind.upper()}_HOME": str(run / kind) for kind in ("config", "data", "cache", "state")}
env |= {"FLATPAK_ID": f"sonora-perf-{side}", "SONORA_LOG": "warn"}

x, xt = C.CDLL("libX11.so.6"), C.CDLL("libXtst.so.6")
x.XOpenDisplay.restype = C.c_void_p
x.XStringToKeysym.restype = C.c_ulong
x.XKeysymToKeycode.argtypes = [C.c_void_p, C.c_ulong]
x.XSync.argtypes = [C.c_void_p, C.c_int]
x.XGetInputFocus.argtypes = [C.c_void_p, C.POINTER(C.c_ulong), C.POINTER(C.c_int)]
x.XGetGeometry.argtypes = [C.c_void_p, C.c_ulong] + [C.c_void_p] * 7
x.XWarpPointer.argtypes = [C.c_void_p, C.c_ulong, C.c_ulong] + [C.c_int] * 2 + [C.c_uint] * 2 + [C.c_int] * 2
xt.XTestFakeKeyEvent.argtypes = [C.c_void_p, C.c_uint, C.c_int, C.c_ulong]
xt.XTestFakeButtonEvent.argtypes = [C.c_void_p, C.c_uint, C.c_int, C.c_ulong]


class XImage(C.Structure):
    _fields_ = [(name, C.c_int) for name in ("width", "height", "xoffset", "format")] + [("data", C.c_void_p)] + [
        (name, C.c_int) for name in ("byte_order", "bitmap_unit", "bitmap_bit_order", "bitmap_pad", "depth",
                                     "bytes_per_line", "bits_per_pixel")]


x.XGetImage.argtypes = [C.c_void_p, C.c_ulong, C.c_int, C.c_int, C.c_uint, C.c_uint, C.c_ulong, C.c_int]
x.XGetImage.restype = C.POINTER(XImage)
x.XDestroyImage.argtypes = [C.POINTER(XImage)]
display = x.XOpenDisplay(None)
if not display:
    raise RuntimeError("cannot open the XWayland display")


def niri(*arguments):
    return sp.check_output(["niri", "msg", *arguments])


def focus(window):
    niri("action", "focus-window", "--id", str(window))
    time.sleep(0.15)


def press(*names):
    keys = [x.XKeysymToKeycode(display, x.XStringToKeysym(name.encode())) for name in names]
    for key in keys:
        xt.XTestFakeKeyEvent(display, key, 1, 0)
    x.XSync(display, 0)
    time.sleep(0.03)
    for key in reversed(keys):
        xt.XTestFakeKeyEvent(display, key, 0, 0)
    x.XSync(display, 0)


def type_text(text):
    for character in text:
        press("space" if character == " " else character)
        time.sleep(0.03)


def geometry():
    """The focused window and its size."""
    window, revert = C.c_ulong(), C.c_int()
    x.XGetInputFocus(display, C.byref(window), C.byref(revert))
    width, height = C.c_uint(), C.c_uint()
    scratch = [C.c_ulong(), C.c_int(), C.c_int(), C.c_uint(), C.c_uint()]
    if window.value <= 1 or not x.XGetGeometry(display, window, C.byref(scratch[0]), C.byref(scratch[1]),
                                                 C.byref(scratch[2]), C.byref(width), C.byref(height),
                                                 C.byref(scratch[3]), C.byref(scratch[4])):
        return None
    return window.value, width.value, height.value


def pointer(window, across, down):
    focus(window)
    target, width, height = geometry()
    x.XWarpPointer(display, 0, target, 0, 0, 0, 0, int(width * across), int(height * down))
    x.XSync(display, 0)


def click(window, across, down):
    pointer(window, across, down)
    xt.XTestFakeButtonEvent(display, 1, 1, 0)
    xt.XTestFakeButtonEvent(display, 1, 0, 0)
    x.XSync(display, 0)


def scroll(window, notches):
    pointer(window, 0.68, 0.65)
    for _ in range(notches):
        xt.XTestFakeButtonEvent(display, 5, 1, 0)
        xt.XTestFakeButtonEvent(display, 5, 0, 0)
        x.XSync(display, 0)
        time.sleep(0.03)


def link(uri):
    sp.Popen([exe, uri], env=env, stdout=sp.DEVNULL, stderr=sp.DEVNULL)


def frame():
    """A coarse grid of the page area's pixels, leaving out the sidebars and the player bar."""
    found = geometry()
    if not found or found[1] < 600 or found[2] < 380:
        return None
    window, width, height = found
    across, down = int(width * 0.68), int(height * 0.70)
    image = x.XGetImage(display, window, int(width * 0.24), int(height * 0.13), across, down, C.c_ulong(-1).value, 2)
    if not image:
        return None
    try:
        picture = image.contents
        if picture.bits_per_pixel != 32:
            return None
        pixels = C.string_at(picture.data, picture.bytes_per_line * picture.height)
        step = 4 * (across // 64)
        rows = (pixels[y * picture.bytes_per_line:(y + 1) * picture.bytes_per_line] for y in range(0, down, down // 46))
        return b"".join(row[channel:across * 4:step] for row in rows for channel in range(3))
    finally:
        x.XDestroyImage(image)


def changed(before, after):
    """Whether enough of the grid moved to count, ignoring compression-level noise."""
    if before is None or after is None:
        return before is not after
    return len(before) != len(after) or sum(abs(a - b) > 14 for a, b in zip(before, after)) > 20


def traffic():
    try:
        return struct.unpack("=QQ", (run / "network.dat").read_bytes())
    except (FileNotFoundError, struct.error):
        return (0, 0)


def settle(app, start, budget, stage, baseline, required=True):
    """Seconds from `start` to the last change on screen, once the screen and the network have
    been quiet for QUIET seconds, or None when that does not happen within `budget`."""
    last_frame, last_traffic = baseline, traffic()
    last_change = None
    quiet_since = time.monotonic()
    while (now := time.monotonic()) - start < budget:
        if app.poll() is not None:
            raise RuntimeError(f"the app exited during {stage}")
        current, sent = frame(), traffic()
        if changed(last_frame, current):
            last_frame, last_change, quiet_since = current, now, now
        if sent != last_traffic:
            last_traffic, quiet_since = sent, now
        if last_change is not None and now - quiet_since >= QUIET:
            return round(last_change - start, 3)
        time.sleep(SAMPLE)
    if last_change is None and required:
        raise RuntimeError(f"nothing changed on screen at {stage}; check the navigation")
    return None


def signed_in():
    return json.loads(sp.check_output([exe, "provider", "--json"], env=env, timeout=3)) == "spotify"


def main():
    known = {window["id"] for window in json.loads(niri("--json", "windows"))}
    launch = env | {"LD_PRELOAD": str(root / "netcount.so"), "SONORA_PROFILE_NETWORK": str(run / "network.dat")}
    with (run / "app.log").open("w") as log, (run / "stages.csv").open("w") as output:
        app = sp.Popen([exe], env=launch, stdout=log, stderr=log)
        launched = time.monotonic()
        try:
            window = None
            while window is None:
                if app.poll() is not None or time.monotonic() - launched > 12:
                    raise RuntimeError("no Sonora window appeared")
                window = next((w["id"] for w in json.loads(niri("--json", "windows"))
                               if w["id"] not in known and (w.get("title") or "").startswith("Sonora")), None)
                time.sleep(0.1)
            niri("action", "set-window-width", "--id", str(window), "1200")
            niri("action", "set-window-height", "--id", str(window), "716")
            focus(window)

            def open_search():
                focus(window)
                press("F8")
                time.sleep(0.4)
                type_text(QUERY[:-1])
                # the query up to its last letter is setup: only the last keystroke is timed
                settle(app, time.monotonic(), 8, "search setup", frame(), required=False)
                shown = frame()
                press(QUERY[-1])
                return shown

            # stage, seconds it may take, and the action that starts it
            stages = [
                ("home_startup", 12, None),
                ("home_scroll", 6, lambda: scroll(window, 32)),
                ("browse_landing", 6, lambda: press("F8")),
                ("search_preview", 6, open_search),
                ("search_results", 6, lambda: press("Return")),
                ("playlist", 6, lambda: link("spotify:playlist:37i9dQZF1DWY4lFlS4Pnso")),
                ("album", 6, lambda: link("spotify:album:4aawyAB9vmqN3uQ7FjRGTy")),
                ("artist", 6, lambda: link("spotify:artist:0gxyHStUsqpMadRV0Di1Qt")),
                ("song", 6, lambda: link("spotify:track:4cOdK2wGLETKBW3PvgPWqT")),
                ("profile", 6, lambda: link("spotify:user:spotify")),
                ("library_playlists", 7, lambda: click(window, 0.043, 0.097)),
            ]
            writer = csv.writer(output)
            writer.writerow(["stage", "settled", "rx", "tx"])
            for stage, budget, action in stages:
                # the screen as it was before the input, so a quick change is not missed
                shown = frame() if action else None
                shown = (action() or shown) if action else None
                start = time.monotonic() if action else launched
                print(side, stage, flush=True)
                settled = settle(app, start, budget, stage, shown)
                if stage == "home_startup" and not signed_in():
                    raise RuntimeError("Spotify did not restore the sign-in, see app.log")
                writer.writerow([stage, settled if settled is not None else "", *traffic()])
        finally:
            if app.poll() is None:
                app.terminate()
                app.wait(timeout=10)


main()
