use bevy::prelude::*;
use crate::components::*;

pub struct PlayerShootingPlugin;
impl Plugin for PlayerShootingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_shooting);
    }
}

fn player_shooting(buttons: Res<ButtonInput<MouseButton>>, query: Query<&Transform, With<Player>>) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    for transform in &query{
        let origin = transform.translation;
        let forward = transform.forward();
        info!("shot fired");
        
        info!("origin: {:?}, direction: {:?}", origin, forward);
    }
}
