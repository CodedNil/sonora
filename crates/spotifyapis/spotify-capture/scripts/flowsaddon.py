OUT=open("/tmp/claude-1000/-home-dan-Downloads/86fe625e-fdc4-4e5f-9e28-5607eeba7215/scratchpad/audio_summary.txt","w")
def response(fl):
    h=fl.request.pretty_host
    if "audio" in h or "heads" in h:
        c=fl.response.raw_content or b""
        OUT.write(f"{fl.request.timestamp_start:.2f} {fl.request.method} {fl.request.pretty_url[:80]} range={fl.request.headers.get('range')} {fl.response.status_code} len={len(c)} {c[:32].hex()}\n"); OUT.flush()
