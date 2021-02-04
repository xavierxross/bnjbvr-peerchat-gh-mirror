// TODO v
const API_BASE = "http://127.0.0.1:8080/api";

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
