import { writable } from 'svelte/store';

const initialNick = "poney" + ((Math.random() * 10000) | 0);
export const nickname = writable(initialNick);

const initialUrl = "https://peertube.fr/videos/watch/07a78392-c981-42af-a7fa-46e23e593716";
export const peertubeUrl = writable(initialUrl);
