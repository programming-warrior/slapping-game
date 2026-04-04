use bevy::prelude::*;

mod camera;
mod world;
mod components;
mod player;
mod combat;


use player::look::PlayerLookPlugin;
use player::movement::PlayerMovementPlugin;
use player::shooting::PlayerShootingPlugin;
use world::WorldPlugin;
use combat::health::HealthPlugin;
// use player_movement::PlayerMovementPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, camera::CameraPlugin,WorldPlugin, PlayerLookPlugin, PlayerMovementPlugin, PlayerShootingPlugin, HealthPlugin ));
    // app.add_systems(Startup, ( spawn_floor, spot_light));
    app.run();
}






