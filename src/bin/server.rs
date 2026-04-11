use bevy::prelude::*;
use bevy_renet::renet::*;
use slapping_game::server::network::{
    new_server, receive_updates, send_server_packets, update_server_transport, NetworkServer,
    NetworkServerTransport, Player, Players,
};
use std::collections::HashMap;

use slapping_game::shared::protocol::ServerMessage;

fn main(){
    let (server, transport) = new_server();
    App::new()
        .add_plugins(MinimalPlugins)
        .insert_resource(server)
        .insert_resource(transport)
        .insert_resource(Players(HashMap::new()))
        .add_systems(PreUpdate, update_server_transport)
        .add_systems(Update, (manage_connections,receive_updates))
        .add_systems(PostUpdate, send_server_packets)
        .run();
}

fn manage_connections(mut server_wrapper: ResMut<NetworkServer>, mut players: ResMut<Players>) {
    let server = &mut server_wrapper.0;
    while let Some(event)  = server.get_event(){
        match event {
            ServerEvent::ClientConnected{client_id} => {
                println!("Client {} connected", client_id);
                players.0.insert(
                    client_id,
                    Player {
                        id: client_id,
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                );

                let payload = bincode::serialize(&ServerMessage::PlayerConnected { id: client_id })
                    .expect("failed to serialize PlayerConnected");
                for target_id in server.clients_id() {
                    if target_id != client_id {
                        server.send_message(target_id, DefaultChannel::ReliableOrdered, payload.clone());
                    }
                }
            }
            ServerEvent::ClientDisconnected {
                client_id,
                reason: _,
            } => {
                println!("Client {} disconnected", client_id);
                players.0.remove(&client_id);

                let payload = bincode::serialize(&ServerMessage::PlayerDisconnected { id: client_id })
                    .expect("failed to serialize PlayerDisconnected");
                for target_id in server.clients_id() {
                    server.send_message(target_id, DefaultChannel::ReliableOrdered, payload.clone());
                }
            }
        }
    }
}