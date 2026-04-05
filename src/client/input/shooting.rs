use bevy::prelude::*;
use crate::shared::components::*;

pub struct PlayerShootingPlugin;
impl Plugin for PlayerShootingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_shooting);
    }
}

fn player_shooting(
    buttons: Res<ButtonInput<MouseButton>>, 
    mut targets: Query<(Entity, &Transform, &mut Health), With<Target>>,
    player: Single<&Transform, With<Player>>
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }
    println!("Player shooting!");
    //direction at which the bullet is moving, which is the forward direction of the player
    let direction = player.forward();
    let shooter_origin = player.translation;

    let mut closest_hit: Option<(Entity, f32)> = None;

    for (entity, target_transform, _) in targets.iter(){
        let target_origin = target_transform.translation;
        let to_target = target_origin - shooter_origin;
        let distance_along_ray = to_target.dot(*direction);
        if distance_along_ray < 0.0 {
            continue; // Target is behind the shooter
        }
        let closest_point = shooter_origin + direction * distance_along_ray;
        let distance_to_target = target_transform.translation.distance(closest_point);

        let hit_radius = 1.0;

        if distance_to_target <= hit_radius {
            let hit_info = (entity, distance_along_ray);
            if let Some((_, closest_distance)) = closest_hit {
                if distance_along_ray < closest_distance {
                    closest_hit = Some(hit_info);
                }
            } else {
                closest_hit = Some(hit_info);
            }
        }

    }

    //apply damage to the closest target hit
    if let Some((entity, _)) = closest_hit {
        if let Ok((_, _, mut health)) = targets.get_mut(entity) {
            health.current -= 25;
            info!("Hit target {:?}, remaining health: {}", entity, health.current);
        }
    }
}
