import {readFileSync, writeFileSync, mkdtempSync} from 'node:fs';
let data = readFileSync(process.argv[2]);
function parse(at) {
    const colon = data.indexOf(58, at);
    if (colon < 0) throw new Error('no size separator');
    const start = colon + 1;
    const end = start + Number(data.toString('ascii', at, colon));
    const chunk = data.subarray(start, end);
    const type = String.fromCharCode(data[end]);
    let value = chunk;
    if (type === '}' || type === ']') {
        const values = [];
        for (let next = start; next < end;) {
            const field = parse(next);
            values.push(field[0]);
            next = field[1];
        }
        value = type === ']' ? values : new Map();
        if (type === '}') for (let i = 0; i < values.length; i += 2)
            value.set(String(values[i]), values[i + 1]);
    } else if (type === '~') value = null;
    else if (type === '#') value = Number(chunk);
    else if (![';', ',', '!', '^'].includes(type)) throw new Error('unknown tnetstring type ' + type);
    return [value, end + 1];
}
const files = new Map();
for (let at = 0; at < data.length;) {
    const [row, next] = parse(at);
    at = next;
    const req = row.get('request'), res = row.get('response');
    if (!(req && res)) continue;
    const path = String(req.get('path'));
    const file = path.match(/\/(?:head|audio|key)\/([0-9a-f]{40})(?:\?|$)/)?.[1];
    if (!file || !res.get('content')) continue;
    const matched = files.get(file) ?? {};
    const headers = res.get('headers') ?? [];
    const range = headers.find(([name]) => String(name).toLowerCase() === 'content-range');
    if (path.startsWith('/head/') && Number(res.get('status_code')) === 200) matched.head = res.get('content');
    if (path.startsWith('/audio/') && range && String(range[1]).startsWith('bytes 0-')) matched.cdn = res.get('content');
    if (path.startsWith('/playplay/v1/key/') && Number(res.get('status_code')) === 200) matched.license = res.get('content');
    files.set(file, matched);
}
const fixtures = [...files.values()].filter(file => file.head && file.cdn && file.license);
console.log('complete triplets:', fixtures.length);
if (fixtures.length) {
    const dir = mkdtempSync('/tmp/sonora-playplay-fixtures.');
    for (const [index, fixture] of fixtures.entries()) {
        console.log('fixture', index + 1, 'head', fixture.head.length, 'encrypted CDN', fixture.cdn.length, 'license', fixture.license.length, 'Ogg head', String(fixture.head.subarray(0,4)) === 'OggS');
        for (const [kind, bytes] of Object.entries(fixture)) writeFileSync(dir + '/' + (index + 1) + '-' + kind, bytes, {mode: 0o600});
    }
    console.log('private fixture directory:', dir);
}
