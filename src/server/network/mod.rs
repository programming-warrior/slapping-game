use bevy::prelude::*;
use bevy_renet::netcode::{NetcodeServerTransport, ServerAuthentication, ServerConfig};
use bevy_renet::renet::{ConnectionConfig, DefaultChannel, RenetServer};
use std::net::UdpSocket;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::shared::protocol::{PROTOCOL_ID, SERVER_ADDR, ClientMessage};

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

pub fn receive_updates(mut server: ResMut<NetworkServer>) {
    for client_id in server.0.clients_id() {
        while let Some(message) = server.0.receive_message(client_id, DefaultChannel::ReliableOrdered) {
            let msg: ClientMessage = bincode::deserialize(&message).unwrap();
            match msg {
                ClientMessage::Input { forward, right } => {
                    println!("Client {} input: forward {}, right {}", client_id, forward, right);
                }
            }
        }
    }
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
