# mitmproxy addon: append every event as JSONL (HTTP, websocket, raw TCP, TLS failures)
import json, time, base64
from mitmproxy import ctx
OUT = open("/home/dan/Downloads/spotify-capture/events.jsonl", "a", buffering=1)
def b(x):
    if x is None: return None
    try: return {"text": x.decode("utf-8")}
    except Exception: return {"b64": base64.b64encode(x).decode()}
def w(d): d["ts"]=time.time(); OUT.write(json.dumps(d)+"\n")
def response(flow):
    r=flow.request; s=flow.response
    w({"type":"http","method":r.method,"url":r.pretty_url,"http_version":r.http_version,
       "req_headers":list(r.headers.items(multi=True)),"req_body":b(r.raw_content) if len(r.raw_content or b"")<2_000_000 else {"omitted":len(r.raw_content)},
       "status":s.status_code,"resp_headers":list(s.headers.items(multi=True)),
       "resp_body":b(s.content) if len(s.raw_content or b"")<200_000 else {"omitted":len(s.raw_content)},
       "t_start":r.timestamp_start})
def error(flow): w({"type":"http_error","url":flow.request.pretty_url,"err":str(flow.error)})
def websocket_message(flow):
    m=flow.websocket.messages[-1]
    w({"type":"ws","url":flow.request.pretty_url,"from_client":m.from_client,"is_text":m.is_text,"data":b(m.content)})
def tcp_message(flow):
    m=flow.messages[-1]
    w({"type":"tcp","server":str(flow.server_conn.address),"sni":flow.server_conn.sni,"from_client":m.from_client,"data":b(m.content)})
def tls_failed_client(data):
    w({"type":"tls_failed_client","sni":data.context.client.sni,"err":str(data.conn.error)})
def tls_failed_server(data):
    w({"type":"tls_failed_server","sni":data.context.server.sni,"err":str(data.conn.error)})
