use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, (spawn_camera, spawn_floor, spot_light, spawn_player));
    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(-2.5, 4.0, 9.0)).looking_at(Vec3::ZERO, Vec3::Y),
    ));
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

fn spawn_player(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let player_mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let player_material = materials.add(Color::srgb(0.8, 0.2, 0.2));
    commands.spawn((
        Mesh3d(player_mesh),
        MeshMaterial3d(player_material),
        Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
    ));
}
