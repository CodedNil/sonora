import json, sys, re, base64, subprocess, os
ev=[json.loads(l) for l in open(sys.argv[1])]
out=sys.argv[2]
pat=re.compile(sys.argv[3])
def body(x):
    if not x: return b""
    if "text" in x: return x["text"].encode()
    if "b64" in x: return base64.b64decode(x["b64"])
    return b""
def raw(bb):
    try:
        r=subprocess.run(["protoc","--decode_raw"],input=bb,capture_output=True,timeout=10)
        if r.returncode==0 and r.stdout: return r.stdout.decode(errors="replace")
    except Exception as e: pass
    return None
def show(bb, ctype):
    if not bb: return "(empty)"
    try:
        t=bb.decode()
        if t.strip().startswith(("{","[")):
            try: return json.dumps(json.loads(t),indent=1)[:6000]
            except: pass
        if all(c.isprintable() or c in "\r\n\t" for c in t[:500]): return t[:4000]
    except: pass
    d=raw(bb)
    return ("[protobuf decode_raw]\n"+d[:6000]) if d else "[binary %d bytes] %s"%(len(bb), bb[:96].hex())
n=int(sys.argv[4]) if len(sys.argv)>4 else 3
i=0
with open(out,"w") as f:
  for e in ev:
    if e["type"]=="http" and pat.search(e["url"]):
      i+=1
      if i>n: continue
      h=dict((k.lower(),v) for k,v in e["req_headers"])
      f.write("="*100+f"\n{e['method']} {e['url']}  [{e['http_version']}] ts={e['t_start']:.3f}\n-- request headers\n")
      for k,v in e["req_headers"]: f.write(f"{k}: {v}\n")
      f.write("-- request body\n"+show(body(e["req_body"]),h.get("content-type",""))+"\n")
      f.write(f"-- response {e['status']}\n")
      for k,v in e["resp_headers"]: f.write(f"{k}: {v}\n")
      f.write("-- response body\n"+show(body(e["resp_body"]),"")+"\n")
  f.write(f"\n# total matches: {i}\n")
