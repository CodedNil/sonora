import ctypes as C
import csv
import json
import os
from pathlib import Path
import struct
import subprocess as sp
import sys
import time

root = Path(sys.argv[1])
kind = sys.argv[2]
run = root / 'results' / f'gui-{kind}'
config, data, cache, state = [run / name for name in ('config', 'data', 'cache', 'state')]
(config / 'sonora').mkdir(parents=True)
(config/'sonora'/'settings.json').write_text(json.dumps({
    'startup':'home', 'provider':'spotify','tray_icon':False,'close_to_tray':False,'spotify_connect':False,
    'lyrics_providers':[], 'keybinds':{'open-search':['f8']},
}))
env = os.environ.copy()
for key in ('WAYLAND_DISPLAY','WAYLAND_SOCKET'):
    env.pop(key, None)
env.update(XDG_CONFIG_HOME=str(config),XDG_DATA_HOME=str(data),XDG_CACHE_HOME=str(cache),XDG_STATE_HOME=str(state),
    FLATPAK_ID=f'sonora-perf-{kind}', SONORA_LOG='warn')
launch = env | {'LD_PRELOAD':str(root/'netcount.so'),'SONORA_PROFILE_NETWORK':str(run/'network.dat')}
exe = root / f'sonora-{kind}'
def windows():
    return json.loads(sp.check_output(['niri','msg','--json','windows']))
def window_action(action, window, *arguments):
    sp.run(['niri', 'msg', 'action', action, '--id', str(window), *arguments], check=True)

before = {w['id'] for w in windows()}
x=C.CDLL('libX11.so.6')
xt=C.CDLL('libXtst.so.6')
x.XOpenDisplay.argtypes=[C.c_char_p]
x.XOpenDisplay.restype=C.c_void_p
x.XStringToKeysym.argtypes=[C.c_char_p]
x.XStringToKeysym.restype=C.c_ulong
x.XKeysymToKeycode.argtypes=[C.c_void_p,C.c_ulong]
x.XGetInputFocus.argtypes=[C.c_void_p,C.POINTER(C.c_ulong),C.POINTER(C.c_int)]
x.XSync.argtypes=[C.c_void_p,C.c_int]
xt.XTestFakeKeyEvent.argtypes=[C.c_void_p,C.c_uint,C.c_int,C.c_ulong]
d=x.XOpenDisplay(None)
if not d:
    raise RuntimeError('Cannot open the XWayland display')
def press(*names):
    keys=[x.XKeysymToKeycode(d,x.XStringToKeysym(n.encode())) for n in names]
    for k in keys: xt.XTestFakeKeyEvent(d,k,1,0)
    x.XSync(d,0)
    time.sleep(.04)
    for k in reversed(keys): xt.XTestFakeKeyEvent(d,k,0,0)
    x.XSync(d,0)
def search(window):
    window_action('focus-window', window)
    time.sleep(.25)
    if not d or not any(w['id']==window and w['is_focused'] for w in windows()):
        raise RuntimeError('Sonora did not receive X11 keyboard focus')
    press('F8')
    time.sleep(.45)
    for ch in 'rick astley':
        press('space' if ch==' ' else ch)
        time.sleep(.035)
def link(uri):
    sp.Popen([exe,uri],env=env,stdout=sp.DEVNULL,stderr=sp.DEVNULL)
x.XGetGeometry.argtypes=[C.c_void_p,C.c_ulong,C.POINTER(C.c_ulong),C.POINTER(C.c_int),C.POINTER(C.c_int),C.POINTER(C.c_uint),C.POINTER(C.c_uint),C.POINTER(C.c_uint),C.POINTER(C.c_uint)]
x.XWarpPointer.argtypes=[C.c_void_p,C.c_ulong,C.c_ulong,C.c_int,C.c_int,C.c_uint,C.c_uint,C.c_int,C.c_int]
xt.XTestFakeButtonEvent.argtypes=[C.c_void_p,C.c_uint,C.c_int,C.c_ulong]
class XImage(C.Structure):
    _fields_ = [(name,C.c_int) for name in ('width','height','xoffset','format')] + [
        ('data',C.c_void_p),
    ] + [(name,C.c_int) for name in ('byte_order','bitmap_unit','bitmap_bit_order','bitmap_pad','depth','bytes_per_line','bits_per_pixel')]
x.XGetImage.argtypes=[C.c_void_p,C.c_ulong,C.c_int,C.c_int,C.c_uint,C.c_uint,C.c_ulong,C.c_int]
x.XGetImage.restype=C.POINTER(XImage)
x.XDestroyImage.argtypes=[C.POINTER(XImage)]
def geometry():
    focus=C.c_ulong()
    revert=C.c_int()
    x.XGetInputFocus(d,C.byref(focus),C.byref(revert))
    if focus.value <= 1:
        raise RuntimeError('no X11 window focus')
    root_x=C.c_ulong()
    left,top=C.c_int(),C.c_int()
    width,height,border,depth=[C.c_uint() for _ in range(4)]
    if not x.XGetGeometry(d,focus,C.byref(root_x),C.byref(left),C.byref(top),C.byref(width),C.byref(height),C.byref(border),C.byref(depth)):
        raise RuntimeError('cannot find window geometry')
    return focus.value,width.value,height.value
def move_pointer(window, relx, rely):
    window_action('focus-window', window)
    time.sleep(.12)
    focus,width,height=geometry()
    x.XWarpPointer(d,0,focus,0,0,0,0,int(width*relx),int(height*rely))
    x.XSync(d,0)
def click_nav(window, fraction):
    move_pointer(window,.047,fraction)
    xt.XTestFakeButtonEvent(d,1,1,0)
    xt.XTestFakeButtonEvent(d,1,0,0)
    x.XSync(d,0)
def scroll(window, count=12):
    move_pointer(window,.68,.65)
    for _ in range(abs(count)):
        button=5 if count > 0 else 4
        xt.XTestFakeButtonEvent(d,button,1,0)
        xt.XTestFakeButtonEvent(d,button,0,0)
        x.XSync(d,0)
        time.sleep(.035)
def frame():
    try:
        focus,width,height=geometry()
    except RuntimeError:
        return None
    if width < 600 or height < 380:
        return None
    w,h=int(width*.68),int(height*.70)
    got=x.XGetImage(d,focus,int(width*.24),int(height*.13),w,h,C.c_ulong(-1).value,2)
    if not got:
        return None
    try:
        img=got.contents
        if img.bits_per_pixel!=32:
            return None
        pixels=C.string_at(img.data,img.bytes_per_line*img.height)
        return bytes(pixels[yy*img.bytes_per_line+xx*4+k] for yy in range(h//46,h,h//46) for xx in range(w//64,w,w//64) for k in range(3))
    finally:
        x.XDestroyImage(got)
def changed(before,after):
    if before is None or after is None:
        return False
    return len(before)!=len(after) or sum(abs(a-b)>14 for a,b in zip(before,after))>20
QUIET = 1.0
SAMPLE = .20

def transfer():
    try:
        return struct.unpack('=QQ', (run / 'network.dat').read_bytes())
    except (FileNotFoundError, struct.error):
        return None

def record_stage(writer, stage, clock, first, last):
    rx, tx = transfer() or (0, 0)
    writer.writerow([stage, first, last, rx, tx, round(time.monotonic()-clock, 2)])


def ready_for_home():
    return json.loads(sp.check_output([exe, 'provider', '--json'], env=env, timeout=3)) == 'spotify'


with (run / 'app.log').open('w') as log, (run / 'stages.csv').open('w') as output:
    app=sp.Popen([exe],env=launch,stdout=log,stderr=log)
    clock=time.monotonic()
    try:
        window=None
        for _ in range(80):
            matches=[w for w in windows() if w['id'] not in before and (w.get('title') or '').startswith('Sonora')]
            if matches:
                window=matches[0]['id']
                break
            if app.poll() is not None:
                raise RuntimeError(f'app exited {app.returncode}')
            time.sleep(.15)
        if window is None:
            raise RuntimeError('no Sonora window')
        window_action('set-window-width', window, '1200')
        window_action('set-window-height', window, '716')
        window_action('focus-window', window)
        print(kind, 'window', window, 'X11 geometry', geometry()[1:], flush=True)
        stages = [
            (10, 'home_startup', None),
            (6, 'home_scroll', lambda: scroll(window, 32)),
            (6, 'browse_landing', lambda: press('F8')),
            (6, 'search_typing', lambda: search(window)),
            (6, 'search_results', lambda: press('Return')),
            (6, 'playlist', lambda: link('spotify:playlist:37i9dQZF1DWY4lFlS4Pnso')),
            (6, 'album', lambda: link('spotify:album:4aawyAB9vmqN3uQ7FjRGTy')),
            (6, 'artist', lambda: link('spotify:artist:0gxyHStUsqpMadRV0Di1Qt')),
            (6, 'song', lambda: link('spotify:track:4cOdK2wGLETKBW3PvgPWqT')),
            (6, 'profile', lambda: link('spotify:user:spotify')),
            (7, 'library_playlists', lambda: click_nav(window, .351)),
        ]
        writer = csv.writer(output)
        writer.writerow(['stage', 'first_visual', 'settled', 'rx', 'tx', 'elapsed'])
        last_frame = last_transfer = None
        for budget, stage, action in stages:
            start = time.monotonic() if action else clock
            if action:
                print(kind, round(start-clock, 2), stage, flush=True)
                action()
            activity = time.monotonic()
            first = last = None
            while True:
                now = time.monotonic()
                if app.poll() is not None:
                    raise RuntimeError(f'app exited {app.returncode} during {stage}')
                if stage == 'home_startup' and 'cannot restore Spotify' in (run / 'app.log').read_text():
                    raise RuntimeError('Spotify rejected the benchmark sign-in. See app.log')
                current = frame()
                if current is not None and (last_frame is None or changed(last_frame, current)):
                    if first is None:
                        first = now-start
                    last, last_frame, activity = now-start, current, now
                traffic = transfer()
                if traffic != last_transfer:
                    last_transfer, activity = traffic, now
                quiet = now-activity >= QUIET and (first is not None or stage == 'home_scroll')
                expired = now-start >= budget
                if quiet or expired:
                    if stage != 'home_startup' or ready_for_home():
                        break
                    if expired:
                        raise RuntimeError('Spotify sign-in did not restore in the benchmark window')
                time.sleep(SAMPLE)
            record_stage(writer, stage, clock, first, last if quiet else None)
    finally:
        if app.poll() is None:
            app.terminate()
            app.wait(timeout=10)
