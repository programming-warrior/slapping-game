use bevy::prelude::Resource;
use bevy_renet::netcode::{ClientAuthentication, NetcodeClientTransport};
use bevy_renet::renet::{ConnectionConfig, RenetClient};
use std::net::UdpSocket;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::shared::protocol::{PROTOCOL_ID, SERVER_ADDR};

#[derive(Resource)]
pub struct NetworkClient(pub RenetClient);

#[derive(Resource)]
pub struct NetworkClientTransport(pub NetcodeClientTransport);

pub fn new_client() -> (NetworkClient, NetworkClientTransport) {
    let server_addr = SERVER_ADDR.parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let client_id = current_time.as_millis() as u64;

    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };

    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
    let client = RenetClient::new(ConnectionConfig::default());

    (NetworkClient(client), NetworkClientTransport(transport))
}
