use std::{
    collections::HashMap,
    mem,
    sync::{Arc, Mutex, Once},
};

use serde::{ser::SerializeStruct, Serialize};
use tide::{security::CorsMiddleware, Body, Request};
use tide_websockets::{Message, WebSocket};

#[derive(Clone, Default)]
struct RoomAnnuaryInner {
    room_to_id: HashMap<String, u32>,
    id_to_room: HashMap<u32, String>,
    next_id: u32,
}

impl RoomAnnuaryInner {
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

#[derive(Clone)]
struct RoomAnnuary {
    inner: Arc<Mutex<RoomAnnuaryInner>>,
}

fn get_annuary() -> RoomAnnuary {
    static mut SINGLETON: *const RoomAnnuary = 0 as *const RoomAnnuary;
    static ONCE: Once = Once::new();

    unsafe {
        ONCE.call_once(|| {
            let singleton = RoomAnnuary {
                inner: Default::default(),
            };
            SINGLETON = mem::transmute(Box::new(singleton));
        });

        (*SINGLETON).clone()
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    // Initialize room annuary.
    let _ = get_annuary();

    // TODO For development purposes only :-)
    app.with(CorsMiddleware::new().allow_origin("*"));

    app.at("/api/room/id").post(get_room_id);
    app.at("/api/room/url").post(get_room_url);
    app.at("/api/chat/:id").get(chat);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

/// Given a URL, what's the id? Will create a new id for the room if needed.
async fn get_room_id(mut req: Request<()>) -> tide::Result {
    let url: String = req.body_string().await?;
    let annuary = get_annuary();
    let room_id = annuary.inner.lock().expect("mutex").url_to_room(&url);
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
async fn get_room_url(mut req: Request<()>) -> tide::Result<Body> {
    let id_str: String = req.body_string().await?;
    let id = if let Ok(id) = id_str.parse::<u32>() {
        id
    } else {
        return Ok(
            Body::from_json(&ApiResult::<&str>::Err("room parameter is not a number")).unwrap(),
        );
    };

    let annuary = get_annuary();
    let api_result = if let Some(url) = annuary.inner.lock().expect("mutex").room_to_url(id) {
        ApiResult::Ok(url.to_string())
    } else {
        ApiResult::Err("unknown room id")
    };
    Ok(Body::from_json(&api_result)?)
}

async fn chat(mut _req: Request<()>) -> tide::Result {
    Ok("jeej".into())
}
