//playing with shapes (Mesh, MeshMaterial) and ButtonInput Resource.
use bevy::prelude::*;

#[derive(Component)]
struct Player;

fn main(){
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup,setup);
    app.add_systems(Update, move_player);
    app.run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2d);
    let shape = meshes.add(Circle::new(50.0));  
    let color = materials.add(Color::hsl(210.0, 0.8, 0.5));
    commands.spawn((
        Mesh2d(shape),
        MeshMaterial2d(color),
        Transform::from_translation(Vec3::new(-100.0, -200.0, 0.0)),
        Player,
    ));
}

fn move_player(
    mut transform: Query<&mut Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    println!("Move Player System Running");
    let speed = 200.0;
    let mut direction = 0.0;

    if keys.pressed(KeyCode::ArrowLeft) {
        direction -= 1.0;
    }
    if keys.pressed(KeyCode::ArrowRight) {
        direction += 1.0;
    }

    for mut t in transform.iter_mut() {
        t.translation.x += direction * speed * time.delta_secs();
    }
}


