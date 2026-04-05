use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Velocity {
    pub speed: f32,
}

#[derive(Component)]
pub struct LookAngles {
    pub yaw: f32,
    pub pitch: f32,
}

#[derive(Component)]
pub struct Target;

#[derive(Component)]
pub struct Health{
    pub current: i32
}
