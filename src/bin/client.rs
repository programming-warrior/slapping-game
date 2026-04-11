use bevy::prelude::*;
use slapping_game::client::render::camera::CameraPlugin;
use slapping_game::combat::health::HealthPlugin;
use slapping_game::client::network::{new_client, NetworkClient, NetworkClientTransport, receive_updates};
use slapping_game::client::input::look::PlayerLookPlugin;
use slapping_game::client::input::movement::PlayerMovementPlugin;
use slapping_game::client::input::shooting::PlayerShootingPlugin;
use slapping_game::client::render::world::WorldPlugin;

fn main() {
    let (client, transport) = new_client();

    App::new()
        .add_plugins((
            DefaultPlugins,
            CameraPlugin,
            WorldPlugin,
            PlayerLookPlugin,
            PlayerMovementPlugin,
            PlayerShootingPlugin,
            HealthPlugin,
        ))
        .insert_resource(client)
        .insert_resource(transport)
        .add_systems(PreUpdate, update_client_transport)
        .add_systems(Update, receive_updates)
        .add_systems(PostUpdate, send_client_packets)
        .run();
}

fn update_client_transport(
    time: Res<Time>,
    mut transport: ResMut<NetworkClientTransport>,
    mut client: ResMut<NetworkClient>,
) {
    transport.0.update(time.delta(), &mut client.0).unwrap();
}

fn send_client_packets(
    mut transport: ResMut<NetworkClientTransport>,
    mut client: ResMut<NetworkClient>,
) {
    transport.0.send_packets(&mut client.0).unwrap();
}
