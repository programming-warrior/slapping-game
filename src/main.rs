use bevy::prelude::*;

mod camera;
mod player;
mod world;


use player::PlayerPlugin;
use world::WorldPlugin;


fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, camera::CameraPlugin, PlayerPlugin, WorldPlugin));
    // app.add_systems(Startup, ( spawn_floor, spot_light));
    app.run();
}






