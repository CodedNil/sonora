import sys,json,base64,gzip; sys.path.insert(0,sys.argv[2]); from pb import *
t0=None
for l in open(sys.argv[1]):
    e=json.loads(l)
    if e["type"]!="ws": continue
    t0=t0 or e["ts"]
    d=e["data"].get("text") or "<bin>"
    try: j=json.loads(d)
    except: print(f"t+{e['ts']-t0:6.1f}", "C>S" if e["from_client"] else "S>C", d[:300]); continue
    # decode payloads
    for p in j.get("payloads",[]) if isinstance(j.get("payloads"),list) else []:
        pass
    s=json.dumps(j)
    if "payloads" in j and j.get("headers",{}).get("Transfer-Encoding")=="gzip":
        try:
            raw=gzip.decompress(base64.b64decode(j["payloads"][0]))
            try: j["payloads_decoded"]=json.loads(raw)
            except: j["payloads_decoded_hex_len"]=len(raw)
        except Exception as ex: j["payloads_decoded"]=str(ex)
        s=json.dumps(j)
    print(f"t+{e['ts']-t0:6.1f}", "C>S" if e["from_client"] else "S>C", s[:int(sys.argv[3])])
