use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup, (spawn_floor, spot_light));
    }
}

fn spot_light(mut commands: Commands) {
    commands.spawn((
        PointLight {
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
    let floor_mesh = meshes.add(Circle::new(4.0));
    let floor_material = materials.add(Color::WHITE);
    commands.spawn((
        Mesh3d(floor_mesh),
        MeshMaterial3d(floor_material),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
}