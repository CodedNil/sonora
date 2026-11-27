import sys,json; sys.path.insert(0,sys.argv[2]); from pb import *
fetched={"517af76aae6c8efcaa9e409fbb7f444a85a10ad5","cbb73fc000ce40915f9a5002aebf892610ed1f15","2b4379a1716d1067637ff272dd233a049448d953","231d036cff1f06bbb33d229efc1b7725ce52dd83","ec7a8d4a0ed90df4aa8f609e55f53d9a1db6f6a4","055150b731095f1c90a2744b60842d751fbcad20","1c950b00c681b117b3fdc3e180457e46d7684162"}
shown=0
def walk(b,depth=0,pre=""):
    try: m=parse(b)
    except Exception: return None
    out=[]
    for f,v in m:
        if isinstance(v,bytes):
            if len(v)==20: out.append(f"{pre}{f}: <fileid {v.hex()}>{' **FETCHED**' if v.hex() in fetched else ''}")
            else:
                sub=walk(v,depth+1,pre+"  ") if depth<4 and len(v)>1 else None
                if sub and not all(c in b" -~" for c in v): out.append(f"{pre}{f} {{"); out+=sub; out.append(pre+"}")
                else: out.append(f"{pre}{f}: {v[:80]!r}")
        else: out.append(f"{pre}{f}: {v}")
    return out
for l in open(sys.argv[1]):
    e=json.loads(l)
    if e["type"]!="http" or "extended-metadata" not in e["url"]: continue
    rq=parse(unz(body(e["req_body"])))
    try: rs=parse(unz(body(e["resp_body"])))
    except Exception: continue
    for f,v in rs:
        if f!=2: continue
        arr=parse(v)
        if get(arr,2)!=5: continue
        for ff,vv in arr:
            if ff!=3: continue
            d=parse(vv); a=get(d,3)
            if not a: continue
            val=get(parse(a),2)
            txt="\n".join(walk(val) or [])
            if shown<3 or "FETCHED" in txt:
                print("ENTITY",get(d,2)); print(txt); print("-"*40); shown+=1
            if shown>8: sys.exit()
