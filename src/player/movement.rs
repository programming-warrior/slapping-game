use bevy::prelude::*;
use bevy_renet::renet::DefaultChannel;

use crate::components::{Player, Velocity};
use crate::network::client::NetworkClient;
use crate::network::protocol::ClientMessage;

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_movement);
    }
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut client: ResMut<NetworkClient>,
    mut query: Query<(&mut Transform, &Velocity), With<Player>>
) {
    for (mut transform, velocity) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        let mut forward = *transform.forward();
        forward.y = 0.0;
        let forward = forward.normalize_or_zero();

        let mut right = *transform.right();
        right.y = 0.0;
        let right = right.normalize_or_zero();

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += forward;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction -= forward;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction -= right;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += right;
        }

        if direction != Vec3::ZERO {
            let speed = velocity.speed;
            transform.translation += direction.normalize() * speed * time.delta_secs();

            //send the message to the server
            let msg = ClientMessage::Input { 
                forward: if keyboard_input.pressed(KeyCode::KeyW) { 1.0 } else if keyboard_input.pressed(KeyCode::KeyS) { -1.0 } else { 0.0 },
                right: if keyboard_input.pressed(KeyCode::KeyD) { 1.0 } else if keyboard_input.pressed(KeyCode::KeyA) { -1.0 } else { 0.0 },
            };
            let serialized_msg = bincode::serialize(&msg).unwrap();
            
            client.0.send_message(DefaultChannel::ReliableOrdered, serialized_msg);
        }
    }
}