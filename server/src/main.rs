use async_std::{channel, prelude::*, task};
use futures::{pin_mut, select, FutureExt};
use serde::{ser::SerializeStruct, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::SystemTime,
};
use tide::{prelude::*, security::CorsMiddleware, Body, Request};
use tide_websockets::{Message as WSMessage, WebSocket, WebSocketConnection};

const DEFAULT_HOST: &'static str = "127.0.0.1";
const DEFAULT_PORT: u16 = 8080;

#[derive(Clone, Default)]
struct RoomAnnuary {
    room_to_id: HashMap<String, u32>,
    id_to_room: HashMap<u32, String>,
    next_id: u32,
}

impl RoomAnnuary {
    fn room_to_url(&self, id: u32) -> Option<&str> {
        self.id_to_room.get(&id).map(|s| s.as_str())
    }
    fn url_to_room(&mut self, url: &str) -> u32 {
        if let Some(id) = self.room_to_id.get(url) {
            return *id;
        }
        let id = self.next_id;
        self.next_id += 1;
        self.room_to_id.insert(url.to_string(), id);
        self.id_to_room.insert(id, url.to_string());
        id
    }
}

#[derive(Default)]
struct ServerStateInner {
    annuary: RoomAnnuary,
    rooms: HashMap<u32, RoomThread>,
}

type ServerState = Arc<Mutex<ServerStateInner>>;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::with_state::<ServerState>(Default::default());

    // TODO For development purposes only :-)
    app.with(CorsMiddleware::new().allow_origin("*"));

    app.at("/api/room/id").post(get_room_id);
    app.at("/api/room/url").post(get_room_url);
    app.at("/api/chat/:id").get(WebSocket::new(chat));

    let host = std::env::var("HOST").unwrap_or(DEFAULT_HOST.to_string());
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|port: String| port.parse::<u16>().ok())
        .unwrap_or(DEFAULT_PORT);

    app.listen((host, port)).await?;
    Ok(())
}

/// Given a URL, what's the id? Will create a new id for the room if needed.
async fn get_room_id(mut req: Request<ServerState>) -> tide::Result {
    let url: String = req.body_string().await?;
    let state = req.state();
    let room_id = state.lock().expect("mutex").annuary.url_to_room(&url);
    Ok(room_id.to_string().into())
}

enum ApiResult<T> {
    Ok(T),
    Err(&'static str),
}

impl<T> Serialize for ApiResult<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("result", 2)?;
        match self {
            ApiResult::Ok(val) => {
                s.serialize_field("status", "ok")?;
                s.serialize_field("value", val)?;
            }
            ApiResult::Err(err) => {
                s.serialize_field("status", "err")?;
                s.serialize_field("msg", err)?;
            }
        }
        s.end()
    }
}

/// Given a room id, what was the associated URL? May fail if the room id didn't exist beforehands.
async fn get_room_url(mut req: Request<ServerState>) -> tide::Result<Body> {
    let id_str: String = req.body_string().await?;
    let id = if let Ok(id) = id_str.parse::<u32>() {
        id
    } else {
        return Ok(
            Body::from_json(&ApiResult::<&str>::Err("room parameter is not a number")).unwrap(),
        );
    };

    let api_result = {
        let annuary = &req.state().lock().expect("mutex").annuary;
        if let Some(url) = annuary.room_to_url(id) {
            ApiResult::Ok(url.to_string())
        } else {
            ApiResult::Err("unknown room id")
        }
    };
    Ok(Body::from_json(&api_result)?)
}

async fn chat(req: tide::Request<ServerState>, stream: WebSocketConnection) -> tide::Result<()> {
    let room_id = req.param("id")?;
    let room_id_num = match room_id.parse::<u32>() {
        Ok(id) => id,
        Err(_) => {
            println!("warning: invalid room id: {}", room_id);
            return Ok(());
        }
    };

    RoomThread::handle_ws(req.state(), room_id_num, stream).await?;
    Ok(())
}

#[derive(Clone, Serialize)]
struct Message {
    author: String,
    time: u128,
    content: String,
}

#[derive(Deserialize)]
struct PreMessage {
    author: String,
    content: String,
}

struct RoomThread {
    stream_tx: channel::Sender<WebSocketConnection>,
    message_tx: channel::Sender<Message>,
}

impl RoomThread {
    async fn handle_ws(
        app_state: &ServerState,
        room_id: u32,
        mut ws: WebSocketConnection,
    ) -> tide::Result<()> {
        let (stream_tx, message_tx) = {
            let mut guard = app_state.lock().expect("mutex");
            let room = guard.rooms.entry(room_id).or_insert_with(|| {
                let (stream_tx, stream_rx) = channel::unbounded::<WebSocketConnection>();
                let (msg_tx, msg_rx) = channel::unbounded::<Message>();

                // Trick: last messages are recorded in json form, to be faster to forward to new
                // clients.
                const NUM_LAST_MSGS: usize = 10;
                let mut last_msgs: Vec<Message> = Vec::with_capacity(NUM_LAST_MSGS);
                let mut last_msgs_i = 0;

                task::spawn(async move {
                    println!("> new task for room {}!", room_id);
                    let mut streams: Vec<WebSocketConnection> = Vec::new();

                    'outer: loop {
                        let new_ws = stream_rx.recv().fuse();
                        let new_message = msg_rx.recv().fuse();

                        pin_mut!(new_ws, new_message);

                        select! {
                            stream_result = new_ws => {
                                if let Ok(stream) = stream_result {
                                    println!("> found new client!");
                                    for msg in &last_msgs {
                                        // Don't bother if the ws closes in the middle.
                                        let json = serde_json::to_string(&msg).unwrap();
                                        if let Err(_) = stream.send_string(json).await {
                                            println!("> ws connection dropped while sending previous messages");
                                            break 'outer;
                                        }
                                    }
                                    streams.push(stream);
                                }
                            }

                            msg_result = new_message => {
                                if let Ok(msg) = msg_result {
                                    println!("> new message in {}: {} says {}", room_id, msg.author, msg.content);

                                    let json = serde_json::to_string(&msg).unwrap();

                                    // Add this message to the list of previous messages.
                                    if last_msgs.len() == NUM_LAST_MSGS {
                                        last_msgs_i = (last_msgs_i + 1) % NUM_LAST_MSGS;
                                        last_msgs[last_msgs_i] = msg.clone();
                                    } else {
                                        last_msgs.push(msg.clone());
                                    }

                                    let mut then_remove = Vec::new();
                                    for (i, stream) in streams.iter().enumerate() {
                                        if let Err(_) = stream.send_string(json.clone()).await {
                                            then_remove.push(i);
                                        }
                                    }

                                    for &i in then_remove.iter().rev() {
                                        println!("> ws connection dropped");
                                        streams.remove(i);
                                    }

                                    // Note: if streams.len() is 0 here, then we could abort the
                                    // task; however this requires the server state to have a
                                    // static lifetime (so it can be accessed from within the
                                    // task), or a different communication channel so *something
                                    // else* can forget about this task. Let this task be dormant
                                    // instead.
                                }
                            }

                            complete => {
                                // Both the stream/msg channels have been closed.
                                break;
                            }
                        }
                    }
                });

                Self {
                    stream_tx,
                    message_tx: msg_tx,
                }
            });

            (room.stream_tx.clone(), room.message_tx.clone())
        };

        stream_tx.send(ws.clone()).await?;

        while let Some(Ok(WSMessage::Text(input))) = ws.next().await {
            let PreMessage { author, content } = if let Ok(json) = serde_json::from_str(&input) {
                json
            } else {
                println!("malformed ws input: {}", input);
                continue;
            };

            // number of milliseconds since 1970-01-01.
            let time = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis();

            message_tx
                .send(Message {
                    author,
                    time,
                    content,
                })
                .await?;
        }

        Ok(())
    }
}
