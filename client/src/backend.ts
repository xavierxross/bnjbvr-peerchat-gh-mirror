const API_PROTOCOL = window.location.protocol;
const API_DOMAIN = window.location.host;
const API_BASE = `//${API_DOMAIN}/api`;

export async function fetchRoomId(url: string): Promise<number> {
    let response = await fetch(API_BASE + "/room/id", {
        method: 'post',
        body: url,
    });
    let id: string = await response.text();
    let numId: number = Number.parseInt(id);
    if (Number.isNaN(numId)) {
        throw new Error("unexpected reponse from the server: non-numeric id!");
    }
    return numId;
}

export async function fetchRoomUrl(id: number): Promise<string> {
    let response = await fetch(API_BASE + "/room/url", {
        method: 'post',
        body: String(id),
    });
    let json = await response.json();
    if (json.status === "err") {
        throw new Error("Backend error: " + json.msg);
    }
    return json.value;
}

export function chatWebsocket(roomId: number) {
    let protocol = API_PROTOCOL.startsWith("https")?"wss":"ws";
    let url = `${protocol}://${API_DOMAIN}/api/chat/${roomId}`;
    return new WebSocket(url);
}
