import sys,json,collections; sys.path.insert(0,sys.argv[2]); from pb import *
kinds=collections.Counter(); types={}
tracks=[]
for l in open(sys.argv[1]):
    e=json.loads(l)
    if e["type"]!="http" or "extended-metadata" not in e["url"]: continue
    rq=parse(unz(body(e["req_body"])))
    for f,v in rq:
        if f==2:
            m=parse(v); uri=get(m,1).decode()
            for ff,vv in m:
                if ff==2: kinds[(uri.split(":")[1], get(parse(vv),1))]+=1
    try: rs=parse(unz(body(e["resp_body"])))
    except Exception as ex: continue
    for f,v in rs:
        if f!=2: continue
        arr=parse(v); k=get(arr,2)
        for ff,vv in arr:
            if ff!=3: continue
            d=parse(vv); uri=(get(d,2) or b"").decode()
            anyb=get(d,3)
            if not anyb: continue
            a=parse(anyb); tu=(get(a,1) or b"").decode()
            types[k]=tu
            if k==10 and len(tracks)<3: tracks.append((uri,get(a,2)))
print("request (entity type, extension_kind) counts:"); [print(" ",k,c,types.get(k[1],"")) for k,c in sorted(kinds.items(),key=lambda x:-x[1])]
for uri,v in tracks:
    t=parse(v)
    print("\nTRACK",uri, "name=",get(t,2), "dur=",get(t,7))
    for f,x in t:
        if f in (12,13,17,25,26,27,28,29,30,31,32,33):
            try:
                p=parse(x); print("  field",f, [(ff, vv.hex() if isinstance(vv,bytes) and len(vv)==20 else vv) for ff,vv in p][:6])
            except Exception: print("  field",f, x[:40])
    print("  all top-level field numbers:", sorted(set(f for f,_ in t)))
