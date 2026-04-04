use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};
use crate::components::{LookAngles, Player, Velocity};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,  spawn_player_camera);
            // .add_systems(Update, camera_follow_player);
    }
}

fn spawn_player_camera(
    mut commands: Commands,
    mut windows: Query<(&mut Window, &mut CursorOptions), With<PrimaryWindow>>,
) {
    if let Ok((_window, mut cursor_options)) = windows.single_mut() {
        cursor_options.visible = false; // Hide the cursor for better immersion
        cursor_options.grab_mode = CursorGrabMode::Locked;
    }

    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0.0, 2.0, 5.0)).looking_at(Vec3::ZERO, Vec3::Y),
        Player,
        Velocity { speed: 6.0 },
        LookAngles { yaw: 0.0, pitch: 0.0 },
    ));
}

// fn spawn_camera(mut commands: Commands) {
//     commands.spawn((
//         Camera3d::default(),
//         Transform::from_translation(Vec3::new(-2.5, 4.0, 9.0)).looking_at(Vec3::ZERO, Vec3::Y),
//     ));
// }

// fn camera_follow_player(
//     player: Single<&Transform, With<Player>>,
//     mut camera: Single<&mut Transform, (With<Camera3d>, Without<Player>)>,
// ) {
//     let player_transform = player.into_inner();
//     let mut camera_transform = camera.into_inner();
    
//     // Calculate camera offset relative to player's rotation
//     // Back = negative forward direction, so we negate it
//     let back = -player_transform.forward();
//     let up = Vec3::Y;
    
//     // Position camera behind and above the player
//     let horizontal_distance = 9.0;
//     let vertical_offset = 4.0;
//     let side_offset = -2.5;
    
//     // Calculate the right direction for side offset
//     let right = player_transform.right();
    
//     // Combine all offsets relative to player's rotation
//     let offset = back.as_vec3() * horizontal_distance + up * vertical_offset + right.as_vec3() * side_offset;
//     let target_position = player_transform.translation + offset;
    
//     // Update camera position
//     camera_transform.translation = target_position;
    
//     // Look at the player
//     camera_transform.look_at(player_transform.translation, Vec3::Y);
// }