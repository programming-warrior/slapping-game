use bevy::prelude::*;
use bevy_renet::netcode::{ClientAuthentication, NetcodeClientTransport};
use bevy_renet::renet::{ConnectionConfig, DefaultChannel, RenetClient};
use std::collections::{HashMap, HashSet};
use std::net::UdpSocket;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::shared::components::Player;
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
    local_client_id: Res<LocalClientId>,
    mut query: Query<(Entity, &mut Transform, &Player)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let client = &mut client_wrapper.0;

    let mut existing_players: HashMap<u64, Entity> = query
        .iter()
        .map(|(entity, _, player)| (player.id, entity))
        .collect();

    println!("Existing players[1]: {:?}", existing_players.keys());
    println!("processing reliable channel");
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        let msg = bincode::deserialize::<ServerMessage>(&message).unwrap();
        if let ServerMessage::PlayerDisconnected { id } = msg {
            println!("Player {} disconnected", id);
            if let Some(&entity) = existing_players.get(&id) {
                println!("found the player in the existing_players");
                commands.entity(entity).despawn();
            }
        }
        if let ServerMessage::PlayerConnected { id } = msg {
            println!("Player {} connected", id);
            let entity = commands.spawn((
                Mesh3d(meshes.add(Mesh::from(Capsule3d::new(0.5, 1.0)))),
                MeshMaterial3d(materials.add(Color::WHITE)),
                Transform::from_xyz(0.0, 0.0, 0.0),
                Player { id },
            )).id();
            existing_players.insert(id, entity); 
        }
    }

    println!("Existing players[2]: {:?}", existing_players.keys());
    println!("processing unreliable channel");
    // Only process the most recent unreliable GameState message per frame to avoid backlog jitter.
    let mut last_msg: Option<ServerMessage> = None;
    while let Some(message) = client.receive_message(DefaultChannel::Unreliable) {
        last_msg = Some(bincode::deserialize(&message).unwrap());
    }

    if let Some(msg) = last_msg {
        match msg {
            ServerMessage::GameState { players } => {
                println!("Received game state with {:?}", players);
                for player in players {
                    let server_translation = Vec3::new(player.x, player.y, player.z);
                    let server_rotation = Quat::from_axis_angle(Vec3::Y, player.look.0)
                        * Quat::from_axis_angle(Vec3::X, player.look.1);

                    if let Some(&entity) = existing_players.get(&player.id) {
                        if let Ok((_, mut transform, _)) = query.get_mut(entity) {
                            if player.id == local_client_id.0 {
                                let error = server_translation - transform.translation;
                                let snap_distance = 1.5;

                                if error.length_squared() > snap_distance * snap_distance {
                                    transform.translation = server_translation;
                                } else {
                                    transform.translation =
                                        transform.translation.lerp(server_translation, 0.2);
                                }
                                transform.rotation =
                                    transform.rotation.slerp(server_rotation, 0.35);
                            } else {
                                if transform.translation.distance_squared(server_translation) > 0.0
                                {
                                    println!(
                                        "Updating player {} position to server state",
                                        player.id
                                    );
                                    transform.translation = server_translation;
                                    transform.rotation =
                                        transform.rotation.slerp(server_rotation, 0.45);
                                }
                            }
                        }
                    } else {
                        println!("Spawning new player {:?}", player.id);
                        let entity = commands.spawn((
                            Mesh3d(meshes.add(Mesh::from(Capsule3d::new(0.5, 1.0)))),
                            MeshMaterial3d(materials.add(Color::WHITE)),
                            Transform::from_xyz(player.x, player.y, player.z),
                            Player { id: player.id }
                        )).id();
                        existing_players.insert(player.id, entity);
                    }
                }
            }
            _ => {}
        }
    }
}
