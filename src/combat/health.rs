use bevy::prelude::*;
use crate::shared::components::{Health, Target};
pub struct HealthPlugin;

impl Plugin for HealthPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Update, health_system);
    }
}

fn health_system(mut commands: Commands, mut query: Query<(Entity, &mut Health), With<Target>>) {
    for (entity, mut health) in query.iter_mut() {
        if health.current <= 0 {
            println!("Target {:?} destroyed!", entity);
            commands.entity(entity).despawn();
        }
    }
}