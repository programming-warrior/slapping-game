use bevy::prelude::*;
use bevy_renet::renet::DefaultChannel;

use crate::shared::components::{Player, Velocity};
use crate::client::network::NetworkClient;
use crate::shared::protocol::{ClientMessage, ClientMoveMessage};

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
        let direction = direction.normalize_or_zero();

        if direction != Vec3::ZERO {
            // Move local player immediately for responsive controls.
            transform.translation += direction * velocity.speed * time.delta_secs();
        }

        // Always send current movement intent (including zero) so server can stop movement.
        let msg = ClientMessage::Move(ClientMoveMessage { direction });
        let serialized_msg = bincode::serialize(&msg).unwrap();
        client.0.send_message(DefaultChannel::Unreliable, serialized_msg);
    }
}
