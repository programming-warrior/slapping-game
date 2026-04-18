use bevy::prelude::*;
use bevy_renet::renet::*;
use bevy_renet::RenetServerPlugin;
use bevy_renet::netcode::NetcodeServerPlugin;
use slapping_game::server::network::{
    new_server, send_server_packets, update_server_transport, NetworkServer,
    NetworkServerTransport, Players, PlayerInput, PlayerInputs
};
use std::collections::HashMap;
use slapping_game::shared::protocol::{ClientMessage, PlayerState, ServerMessage};

fn main(){
    let (server, transport) = new_server();
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(RenetServerPlugin)
        .add_plugins(NetcodeServerPlugin)
        .insert_resource(server)
        .insert_resource(transport)
        .insert_resource(Players(HashMap::new()))
        .insert_resource(PlayerInputs(HashMap::new()))
        .add_systems(PreUpdate, update_server_transport)
        .add_systems(Update, (manage_connections,receive_updates, update_players, broadcast_updates).chain())
        .add_systems(PostUpdate, send_server_packets)
        .run();
}

fn manage_connections(
    mut server_wrapper: ResMut<NetworkServer>,
    mut players: ResMut<Players>,
    mut inputs: ResMut<PlayerInputs>,
) {
    let server = &mut server_wrapper.0;
    while let Some(event)  = server.get_event(){
        match event {
            ServerEvent::ClientConnected{client_id} => {
                println!("Client {} connected", client_id);
                players.0.insert(
                    client_id,
                    PlayerState {
                        id: client_id,
                        x: 0.0,
                        y: 2.0,
                        z: 5.0,
                        look: (0.0, 0.0),
                    },
                );

                let payload = bincode::serialize(&ServerMessage::PlayerConnected { id: client_id })
                    .expect("failed to serialize PlayerConnected");
                // server.broadcast_message(DefaultChannel::ReliableOrdered, payload.clone());
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
                inputs.0.remove(&client_id);

                let payload = bincode::serialize(&ServerMessage::PlayerDisconnected { id: client_id })
                    .expect("failed to serialize PlayerDisconnected");
                for target_id in server.clients_id() {
                    server.send_message(target_id, DefaultChannel::ReliableOrdered, payload.clone());
                }
            }
        }
    }
}



pub fn receive_updates(mut server: ResMut<NetworkServer>, mut inputs: ResMut<PlayerInputs>, mut players: ResMut<Players>) {
    for client_id in server.0.clients_id() {
        while let Some(message) = server.0.receive_message(client_id, DefaultChannel::Unreliable) {
            let msg: ClientMessage = bincode::deserialize(&message).unwrap();
            println!("Received message from client {}: {:?}", client_id, msg);
            match msg {
                ClientMessage::Move(m) => {
                    let look = if let Some(previous_input) = inputs.0.get(&client_id) {
                        previous_input.look
                    } else if let Some(player_state) = players.0.get(&client_id) {
                        player_state.look
                    } else {
                        (0.0, 0.0)
                    };
            
                    inputs.0.insert(client_id, PlayerInput {
                        id: client_id,
                        direction: m.direction,
                        look,
                    });
                }
                ClientMessage::Look(m) => {
                    let direction = if let Some(previous_input) = inputs.0.get(&client_id) {
                        previous_input.direction
                    } else {
                        Vec3::ZERO
                    };
                    
                    inputs.0.insert(client_id, PlayerInput {
                        id: client_id,
                        direction,
                        look: (m.0, m.1),
                    });
                }
            }
        }
    }
}

pub fn update_players(mut players: ResMut<Players>, inputs: ResMut<PlayerInputs>, time: Res<Time>) {
    const SPEED: f32 = 6.0; //MOVEMENT SPEED OF THE PLAYER
    for(client_id, input) in inputs.0.iter(){
        if let Some(player) = players.0.get_mut(client_id){
            player.x += input.direction.x * SPEED * time.delta_secs();
            player.z += input.direction.z * SPEED * time.delta_secs();
            //yaw
            player.look.0 = input.look.0;
            //pitch
            player.look.1 = player.look.1.clamp(-std::f32::consts::FRAC_PI_2 + 0.01, std::f32::consts::FRAC_PI_2 - 0.01);
            player.look.1 = input.look.1;
        }
    }
}


pub fn broadcast_updates(mut server_wrapper: ResMut<NetworkServer>, players: Res<Players>) {
    if players.0.is_empty() {
        return;
    }
    let server= &mut server_wrapper.0;
    let game_state: Vec<PlayerState> = players.0.iter().map(|(_client_id, state): (&u64, &PlayerState)| state.clone()).collect(); 
    let msg: ServerMessage = ServerMessage::GameState { players: game_state };
    let serialized_msg = bincode::serialize(&msg).unwrap();

    println!("Broadcasting game state to {} clients", server.clients_id().len());
    server.broadcast_message(DefaultChannel::ReliableOrdered, serialized_msg);
}