use bevy::prelude::*;

mod camera;
mod player;
mod world;
mod player_movement;


use player::PlayerPlugin;
use world::WorldPlugin;
use player_movement::PlayerMovementPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, camera::CameraPlugin, PlayerPlugin, WorldPlugin,PlayerMovementPlugin));
    // app.add_systems(Startup, ( spawn_floor, spot_light));
    app.run();
}






