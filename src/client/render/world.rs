use bevy::prelude::*;
use bevy::math::primitives::Cuboid;

pub struct WorldPlugin;

use crate::shared::components::{Health, Target};

impl Plugin for WorldPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup, (spawn_floor, spot_light, spawn_targets));
    }
}

fn spot_light(mut commands: Commands) {
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true, // Turn on shadows for this light
            ..default()
        },
        Transform::from_xyz(0.0, 8.0, 4.0),
    ));
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor_mesh = meshes.add(Plane3d::default().mesh().size(20.0, 20.0));
let floor_material = materials.add(Color::WHITE);
    commands.spawn((
        Mesh3d(floor_mesh),
        MeshMaterial3d(floor_material),
    ));

}


fn spawn_targets(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>){
    for i in 0..5 {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::WHITE)),
            Transform::from_xyz(i as f32 *  4.0, 1.0, -5.0),
            Target,
             Health { current: 100 },
        ));
    }
}
