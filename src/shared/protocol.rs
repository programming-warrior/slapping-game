use serde::{Serialize, Deserialize};

pub const PROTOCOL_ID: u64 = 7;
pub const SERVER_ADDR: &str = "127.0.0.1:5000";

#[derive(Serialize, Deserialize)]
pub enum ClientMessage {
    Input { forward: f32, right: f32},
}

#[derive(Serialize, Deserialize)]
pub enum ServerMessage {
    PlayerPosition { id: u64, x: f32, y: f32, z: f32 },
    PlayerConnected { id: u64 },
    PlayerDisconnected { id: u64 },
}
