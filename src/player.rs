use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup, spawn_player);
    }
}

fn spawn_player(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let player_mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let player_material = materials.add(Color::srgb(0.8, 0.2, 0.2));
    commands.spawn((
        Mesh3d(player_mesh),
        MeshMaterial3d(player_material),
        Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
    ));
}