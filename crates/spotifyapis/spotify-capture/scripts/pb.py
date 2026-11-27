import gzip, json, base64, sys
def varint(b,i):
    r=s=0
    while True:
        c=b[i]; i+=1; r|=(c&0x7f)<<s; s+=7
        if c<0x80: return r,i
def parse(b):
    i=0; out=[]
    while i<len(b):
        k,i=varint(b,i); f,t=k>>3,k&7
        if t==0: v,i=varint(b,i)
        elif t==1: v=b[i:i+8]; i+=8
        elif t==5: v=b[i:i+4]; i+=4
        elif t==2:
            l,i=varint(b,i); v=b[i:i+l]; i+=l
        else: raise ValueError("wt%d"%t)
        if f==0: raise ValueError
        out.append((f,v))
    return out
def get(m,*path):
    for n,p in enumerate(path):
        m=[v for f,v in m if f==p]
        if not m: return None
        m=m[0]
        if n<len(path)-1: m=parse(m)
    return m
def unz(b):
    return gzip.decompress(b) if b[:2]==b"\x1f\x8b" else b
def body(x):
    if not x: return b""
    if "text" in x: return x["text"].encode()
    if "b64" in x: return base64.b64decode(x["b64"])
    return b""
