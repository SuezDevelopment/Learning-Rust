use actix_web::{ web, Error, HttpRequest, HttpResponse };
use actix_web_actors::ws;
use actix::{ Actor, StreamHandler};

use uuid::uuid;

use serde::{ Deserialize, Serialize };
use serde_json::json;

use chrono::{DateTime, Local};

use crate::middleware::*;
use crate::response::*;

#[derive(Deserialize, Serialize)]
struct WsMessage {
    action: String,
    data: serde_json::Value,
}

#[derive(Clone, Copy)]
pub struct WsConnection {
    id: uuid::Uuid,
    connected_at: std::time::SystemTime,
}


impl WsConnection {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            connected_at: std::time::SystemTime::now(),
            // client_info: None,
            // message_count: 0,
            // last_ping: std::time::SystemTime::now(),
            // room: None,
        }
    }

    pub fn send_welcome_message(&self, ctx: &mut ws::WebsocketContext<Self>) {
        let welcome_message = json!({
            "action": "welcome",
            "data": {
                "connection_id": self.id.to_string(),
                "message": "Successfully connected to WebSocket server"
            }
        });
        
        ctx.text(welcome_message.to_string());
    }
}

impl Actor for WsConnection {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConnection {
    fn started(&mut self, ctx: &mut Self::Context) {
        self.send_welcome_message(ctx);
    }

    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                if let Ok(ws_message) = serde_json::from_str::<WsMessage>(&text) {
                    let response = match ws_message.action.as_str() {
                        "get_temperature" =>
                            json!({
                            "action": "temperature_response",
                            "data": { "temperature": 25.5 }
                        }),
                        "get_multiple_temperature" =>
                            json!({
                            "action": "multiple_temperature_response",
                            "data": [{ "temperature": 25.5 }, { "temperature": 25.5 }, { "temperature": 25.5 }]
                        }),
                        "update_settings" =>
                            json!({
                            "action": "settings_updated",
                            "data": ws_message.data
                        }),
                        "ping" =>
                            json!({
                            "action": "pong",
                            "data": null
                        }),
                        _ =>
                            json!({
                            "action": "error",
                            "data": { "message": "Unknown action" }
                        }),
                    };
                    ctx.text(response.to_string());
                }
            }
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
            }
            _ => (),
        }
    }
}

pub async fn websocket_handler(
    req: HttpRequest,
    stream: web::Payload
) -> Result<HttpResponse, Error> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                if validate_token(token) {
                    let connection = WsConnection::new();
                    let local_time: DateTime<Local> = connection.connected_at.into();
                    println!(
                        "New websocket connection established - ID: {} at: {}", 
                        connection.id,
                        local_time.format("%B %d, %Y at %H:%M:%S").to_string()
                    );
                    return ws::start(connection, &req, stream);
                }
            }
        }
    }
    Ok(response_unauthorized("unauthorized: invalid or missing authorization token"))
}
