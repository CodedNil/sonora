// Summarize license, manifest and credential relationships without printing credentials or bodies.
import { readFileSync } from 'node:fs';

const file = process.argv[2];
if (!file) {
    console.error('usage: node scripts/desktop-widevine.mjs events.jsonl');
    process.exit(2);
}

const events = readFileSync(file, 'utf8').split('\n')
    .flatMap(line => {
        try {
            const event = JSON.parse(line);
            return event?.type === 'http' && URL.canParse(event.url) ? [event] : [];
        } catch {
            return [];
        }
    });

function responseJson(event) {
    try {
        const body = event.resp_body;
        return JSON.parse(body.text ?? Buffer.from(body.b64, 'base64').toString('utf8'));
    } catch {
        return null;
    }
}

function header(event, name) {
    return event.req_headers?.find(([key]) => key.toLowerCase() === name)?.[1];
}

function bearer(event) {
    return header(event, 'authorization')?.replace(/^Bearer /i, '');
}

const desktopBearer = new Set(events
    .filter(event => event.method === 'POST' && event.status === 200 &&
        event.url.startsWith('https://accounts.spotify.com/api/token'))
    .map(event => responseJson(event)?.access_token)
    .filter(Boolean));
const nativeAudio = events.filter(event => event.method === 'POST' && event.status === 200 &&
    new URL(event.url).pathname.startsWith('/playplay/v1/key/'));
const nativeClientToken = new Set(nativeAudio.map(event => header(event, 'client-token')).filter(Boolean));
const requests = new Map();

function record(kind, event) {
    const key = [kind, event.method, event.status].join(' ');
    requests.set(key, (requests.get(key) ?? 0) + 1);
}

let clearVideoManifests = 0;
let protectedVideoManifests = 0;
let manifestsWithAudio = 0;
let videoRequestsWithDesktopBearer = 0;
let videoRequestsWithNativeClientToken = 0;
let licensePostsWithDesktopBearer = 0;
let licensePostsWithNativeClientToken = 0;

for (const event of events) {
    const path = new URL(event.url).pathname;
    if (path === '/widevine-license/v1/application-certificate') {
        record('Widevine certificate', event);
    } else if (path === '/melody/v1/license_url') {
        record('Melody license discovery', event);
    } else if (/^\/widevine-license\/v1\/(audio|video)\/license$/.test(path)) {
        record(path.includes('/video/') ? 'Widevine video license' : 'Widevine audio license', event);
        if (event.method === 'POST') {
            if (desktopBearer.has(bearer(event))) licensePostsWithDesktopBearer++;
            if (nativeClientToken.has(header(event, 'client-token'))) licensePostsWithNativeClientToken++;
        }
    } else if (path.startsWith('/manifests/v9/')) {
        record('Video manifest', event);
        if (event.method !== 'GET' || event.status !== 200) continue;
        if (desktopBearer.has(bearer(event))) videoRequestsWithDesktopBearer++;
        if (nativeClientToken.has(header(event, 'client-token'))) videoRequestsWithNativeClientToken++;
        const contents = responseJson(event)?.contents;
        if (!Array.isArray(contents) || contents.length === 0) continue;
        if (contents.some(content => content.encryption_infos?.length)) protectedVideoManifests++;
        else clearVideoManifests++;
        if (contents.some(content => content.profiles?.some(profile => profile.audio_codec))) {
            manifestsWithAudio++;
        }
    }
}

console.log(JSON.stringify({
    requests: Object.fromEntries(requests),
    nativeAudioLicenses: nativeAudio.length,
    clearVideoManifests,
    protectedVideoManifests,
    manifestsWithAudio,
    videoRequestsWithDesktopBearer,
    videoRequestsWithNativeClientToken,
    licensePostsWithDesktopBearer,
    licensePostsWithNativeClientToken,
}, null, 2));
