use bevy::prelude::*;
use crate::player::{Player, Velocity};

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Update, player_movement);
    }
}


fn player_movement(player: Single<(&mut Velocity, &mut Transform), With<Player>>, keys: Res<ButtonInput<KeyCode>>, time: Res<Time> ) {
    const MOVE_SPEED: f32 = 10.0;
    const ROTATION_SPEED: f32 = 2.5;
    let (mut velocity, mut transform) = player.into_inner();
    
    // Handle rotation (A/D keys)
    if keys.pressed(KeyCode::KeyA) {
        // Rotate left (counter-clockwise around Y axis)
        transform.rotate_y(ROTATION_SPEED * time.delta_secs());
    }
    if keys.pressed(KeyCode::KeyD) {
        // Rotate right (clockwise around Y axis)
        transform.rotate_y(-ROTATION_SPEED * time.delta_secs());
    }

    // Reset movement velocity
    let mut forward_movement = 0.0;

    // Handle forward/backward movement (W/S keys)
    if keys.pressed(KeyCode::KeyW) {
        forward_movement += 1.0; // Move forward
    }
    // if keys.pressed(KeyCode::KeyS) {
    //     forward_movement -= 1.0; // Move backward
    // }

    // Calculate movement direction based on player's current rotation
    // transform.forward() returns the direction the player is facing
    let forward_dir = transform.forward();
    println!("Forward direction: {:?}", forward_dir);
    
    // Apply movement in the direction the player is facing
    transform.translation += forward_dir.as_vec3() * forward_movement * MOVE_SPEED * time.delta_secs();
    
    // Update velocity for potential future use
    velocity.x = forward_dir.x * forward_movement;
    velocity.z = forward_dir.z * forward_movement;
}   