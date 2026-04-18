use bevy::prelude::*;
use bevy_renet::netcode::{NetcodeServerTransport, ServerAuthentication, ServerConfig};
use bevy_renet::renet::{ConnectionConfig, DefaultChannel, RenetServer};
use std::collections::HashMap;
use std::net::UdpSocket;
use {serde::{Serialize, Deserialize}};
use std::time::{SystemTime, UNIX_EPOCH};


use crate::shared::protocol::{PROTOCOL_ID, SERVER_ADDR, ClientMoveMessage, ClientLookMessage, PlayerState};

#[derive(Serialize, Deserialize)]
pub struct PlayerInput{
    pub id: u64,
    pub direction: Vec3,
    pub look: (f32, f32), //yaw, pitch
}

#[derive(Resource)]
pub struct PlayerInputs(pub HashMap<u64, PlayerInput>);



#[derive(Resource)]
pub struct Players(pub HashMap<u64, PlayerState>);

#[derive(Resource)]
pub struct NetworkServer(pub RenetServer);

#[derive(Resource)]
pub struct NetworkServerTransport(pub NetcodeServerTransport);

pub fn new_server() -> (NetworkServer, NetworkServerTransport) {
    let public_addr = SERVER_ADDR.parse().unwrap();
    let socket = UdpSocket::bind(public_addr).unwrap();
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let server_config = ServerConfig {
        current_time,
        max_clients: 64,
        protocol_id: PROTOCOL_ID,
        public_addresses: vec![public_addr],
        authentication: ServerAuthentication::Unsecure,
    };

    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
    let server = RenetServer::new(ConnectionConfig::default());

    (NetworkServer(server), NetworkServerTransport(transport))
}


pub fn update_server_transport(
    time: Res<Time>,
    mut transport: ResMut<NetworkServerTransport>,
    mut server: ResMut<NetworkServer>,
) {
    transport.0.update(time.delta(), &mut server.0).unwrap();
}

pub fn send_server_packets(mut transport: ResMut<NetworkServerTransport>, mut server: ResMut<NetworkServer>) {
    transport.0.send_packets(&mut server.0);
}
