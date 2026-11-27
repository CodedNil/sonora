import sys,json; sys.path.insert(0,sys.argv[2]); from pb import *
R={0:"UNKNOWN",1:"SPIRC_HELLO",2:"SPIRC_NOTIFY",3:"NEW_DEVICE",4:"PLAYER_STATE_CHANGED",5:"VOLUME_CHANGED",6:"PICKER_OPENED",7:"BECAME_INACTIVE",8:"ALIVE_TICK",9:"NEW_CONNECTION"}
t0=None
for l in open(sys.argv[1]):
    e=json.loads(l)
    if e["type"]=="http" and "connect-state/v1/devices" in e["url"] and e["method"]=="PUT":
        b=unz(body(e["req_body"])); m=parse(b)
        try:
            ps=get(m,2,2); ps=parse(ps) if isinstance(ps,bytes) else []
        except Exception: ps=[]
        try:
            di=get(m,2,1); di=parse(di) if isinstance(di,bytes) else []
        except Exception: di=[]
        def g(*p):
            v=get(ps,*p) if ps else None
            return v.decode(errors="replace") if isinstance(v,bytes) else v
        t0=t0 or e["t_start"]
        print(f"t+{e['t_start']-t0:6.1f} reason={R.get(get(m,5))} active={get(m,4)} vol={get(di,2) if di else None} name={get(di,3) if di else None} msgid={get(m,6)} lastcmd={get(m,8)} playing={g(12)} paused={g(13)} buffering={g(14)} pos_as_of_ts={g(10)} ts={g(1)} dur={g(11)} uri={g(7,1)} ctx={g(2)} playback_id={(get(ps,8) or b'').hex() if isinstance(get(ps,8),bytes) else g(8)} speed={g(9)}")
