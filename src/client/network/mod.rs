use bevy::prelude::*;
use bevy_renet::netcode::{ClientAuthentication, NetcodeClientTransport};
use bevy_renet::renet::{ConnectionConfig, DefaultChannel, RenetClient};
use std::collections::HashMap;
use std::net::UdpSocket;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::shared::components::{RemotePlayer, Player};
use crate::shared::protocol::{PROTOCOL_ID, SERVER_ADDR, ServerMessage};

#[derive(Resource)]
pub struct NetworkClient(pub RenetClient);

#[derive(Resource)]
pub struct NetworkClientTransport(pub NetcodeClientTransport);

#[derive(Resource)]
pub struct LocalClientId(pub u64);

pub fn new_client() -> (NetworkClient, NetworkClientTransport, LocalClientId) {
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

    (
        NetworkClient(client),
        NetworkClientTransport(transport),
        LocalClientId(client_id),
    )
}


pub fn receive_updates(
    mut client_wrapper: ResMut<NetworkClient>,
    mut query: Query<(Entity, &mut Transform, &mut Player)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let client = &mut client_wrapper.0;

    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        let msg: ServerMessage = bincode::deserialize(&message).unwrap();
        
        println!("Received message from server: {:?}", msg);

        let existing_players: HashMap<u64, Entity> = query
            .iter()
            .map(|(entity, _, player)| (player.id, entity))
            .collect();

        match msg {
            ServerMessage::GameState { players } => {
                for player in players {
                    if let Some(&entity) = existing_players.get(&player.id) {
                        if let Ok((_, mut transform, _)) = query.get_mut(entity) {
                            transform.translation = Vec3::new(player.x, player.y, player.z);
                            transform.rotation = Quat::from_axis_angle(Vec3::Y, player.look.0) * Quat::from_axis_angle(Vec3::X, player.look.1);
                        }
                    }
                }
            }
    
            ServerMessage::PlayerConnected { id } => {
                   commands.spawn((
                        Mesh3d(meshes.add(Mesh::from(Capsule3d::new(0.5, 1.0)))),
                        MeshMaterial3d(materials.add(Color::WHITE)),
                        RemotePlayer { id },
                    ));
            }
            ServerMessage::PlayerDisconnected { id } => {
                println!("Player {} disconnected", id);
                if let Some(&entity) = existing_players.get(&id) {
                    commands.entity(entity).despawn();
                }
            }

        }
    }
}
