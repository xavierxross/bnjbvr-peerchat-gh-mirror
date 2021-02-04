import * as backend from './backend';

// Local cache for room/url mappings.
const ROOM_TO_URL = {};
const URL_TO_ROOM = {};

export function registerRoomUrl(roomId: number, url: string) {
    URL_TO_ROOM[url] = roomId;
    ROOM_TO_URL[roomId] = url;
}

// Returns the room url from the cache, or fetched from the server.
export async function getRoomUrl(roomId: number): Promise<String> {
    if (typeof ROOM_TO_URL[roomId] !== 'undefined') {
        return ROOM_TO_URL[roomId];
    }
    let result = await backend.fetchRoomUrl(roomId);
    registerRoomUrl(roomId, result);
    return result;
}

export async function getRoomId(url: string): Promise<number> {
    if (typeof URL_TO_ROOM[url] !== 'undefined') {
        return URL_TO_ROOM[url];
    }
    let roomId = await backend.fetchRoomId(url);
    registerRoomUrl(roomId, url);
    return roomId;
}
