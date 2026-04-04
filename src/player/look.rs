use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use crate::components::{Player, Velocity, LookAngles};

pub struct PlayerLookPlugin;

impl Plugin for PlayerLookPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_look);
    }
}

fn mouse_look(
    mut mouse_events: MessageReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut LookAngles), With<Player>>
)
{
    let mut delta = Vec2::ZERO;
    for event in mouse_events.read() {
        delta += event.delta;
    }

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

