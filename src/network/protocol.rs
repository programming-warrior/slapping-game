use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum ClientMessage {
    Input { forward: f32, right: f32},
}

#[derive(Serialize, Deserialize)]
pub enum ServerMessage { 
    PlayerPosition {id: u64, x: f32, y: f32, z: f32}
}
