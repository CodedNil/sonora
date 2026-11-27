import sys,json,collections,subprocess; sys.path.insert(0,sys.argv[2]); from pb import *
c=collections.Counter(); ex={}
for l in open(sys.argv[1]):
    e=json.loads(l)
    if e["type"]!="http" or "gabo-receiver" not in e["url"]: continue
    for f,v in parse(unz(body(e["req_body"]))):
        if f!=1: continue
        ev=parse(v); n=get(ev,2).decode(); c[n]+=1
        pl=get(ev,3); 
        if pl: ex.setdefault(n,[]).append(get(parse(pl),2))
for k,n in c.most_common(): print(n,k)
for name in sys.argv[3].split(","):
    for p in ex.get(name,[])[:int(sys.argv[4])]:
        r=subprocess.run(["protoc","--decode_raw"],input=p,capture_output=True)
        print(f"\n### {name}\n"+r.stdout.decode(errors="replace")[:2500])
