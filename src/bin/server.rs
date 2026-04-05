use bevy::prelude::*;

use slapping_game::server::network::{new_server, receive_updates, send_server_packets, update_server_transport};

fn main(){
    let (server, transport) = new_server();
    App::new()
        .add_plugins(MinimalPlugins)
        .insert_resource(server)
        .insert_resource(transport)
        .add_systems(PreUpdate, update_server_transport)
        .add_systems(Update, receive_updates)
        .add_systems(PostUpdate, send_server_packets)
        .run();
}