import { writable } from 'svelte/store';

const initialNick = "poney" + ((Math.random() * 10000) | 0);
export const nickname = writable(initialNick);

const initialUrl = "https://aperi.tube/videos/watch/b25d0548-8247-4073-aeac-776ccdfe5a47";
export const peertubeUrl = writable(initialUrl);
