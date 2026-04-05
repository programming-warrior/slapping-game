use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;
use crate::shared::components::{LookAngles, Player};

pub struct PlayerLookPlugin;

impl Plugin for PlayerLookPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_look);
    }
}

fn mouse_look(
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut query: Query<(&mut Transform, &mut LookAngles), With<Player>>
)
{
    let delta = mouse_motion.delta;

    if delta != Vec2::ZERO {
        let sensitivity = 0.002; // Adjust this for faster/slower look
        for (mut transform, mut angles) in query.iter_mut() {
            // let yaw = Quat::from_rotation_y(-delta.x * sensitivity);
            // let pitch = Quat::from_rotation_x(-delta.y * sensitivity);
            // transform.rotation = yaw * transform.rotation; 
            // transform.rotation = transform.rotation * pitch; 

            angles.yaw -= delta.x * sensitivity;
            angles.pitch -= delta.y * sensitivity;
            angles.pitch = angles.pitch.clamp(-std::f32::consts::FRAC_PI_2 + 0.01, std::f32::consts::FRAC_PI_2 - 0.01); // Prevent flipping

            transform.rotation = Quat::from_axis_angle(Vec3::Y, angles.yaw) * Quat::from_axis_angle(Vec3::X, angles.pitch);
        }
    }
}

// fn sync_look_angles_from_transform(
//     mut query: Query<(&Transform, &mut LookAngles), With<Player>>,
// ) {
//     for (transform, mut angles) in query.iter_mut() {
//         let (yaw, pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
//         angles.yaw = yaw;
//         angles.pitch = pitch;
//     }
// }

