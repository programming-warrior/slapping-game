use serde::{Serialize, Deserialize};
use bevy::prelude::Vec3;

pub const PROTOCOL_ID: u64 = 7;
pub const SERVER_ADDR: &str = "127.0.0.1:5000";

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientMoveMessage {
   pub direction: Vec3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientLookMessage (pub f32, pub f32); //yaw, pitch

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    Move(ClientMoveMessage),
    Look(ClientLookMessage),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerState {
    pub id: u64,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub direction: Vec3,
    pub look: (f32, f32), //yaw, pitch
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    GameState { players: Vec<PlayerState> },
    PlayerConnected { id: u64 },
    PlayerDisconnected { id: u64 },
}
