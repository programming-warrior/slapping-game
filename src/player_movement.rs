use bevy::prelude::*;
use crate::player::{Player, Velocity};

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Update, player_movement);
    }
}


fn player_movement(player: Single<(&mut Velocity, &mut Transform), With<Player>>, keys: Res<ButtonInput<KeyCode>>, time: Res<Time> ) {
    const SPEED: f32 = 10.0;
    let (mut velocity, mut transform) = player.into_inner();
    
    // Reset velocity
    velocity.x = 0.0;
    velocity.z = 0.0;

    // Check for input and set velocity accordingly
    if keys.pressed(KeyCode::KeyW) {
        velocity.z -= 1.0; // Move forward
    }
    if keys.pressed(KeyCode::KeyS) {
        velocity.z += 1.0; // Move backward
    }
    if keys.pressed(KeyCode::KeyA) {
        velocity.x -= 1.0; // Move left
    }
    if keys.pressed(KeyCode::KeyD) {
        velocity.x += 1.0; // Move right
    }

    // Normalize the velocity to prevent faster diagonal movement
    let length = (velocity.x.powi(2) + velocity.z.powi(2)).sqrt();
    if length > 0.0 {
        velocity.x /= length;
        velocity.z /= length;
    }

    // Apply movement to the player's transform
    transform.translation.x += velocity.x * SPEED * time.delta_secs(); // Adjust speed as needed
    transform.translation.z += velocity.z * SPEED * time.delta_secs(); // Adjust speed as needed
  
}   