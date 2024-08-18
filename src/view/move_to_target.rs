use crate::prelude::*;

#[derive(Component)]
pub struct TargetPosition(pub Vec3);
#[derive(Component)]
pub struct MovementSpeed(pub f32);

pub struct MoveToTargetPlugin;

impl Plugin for MoveToTargetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                move_to_target,
                remove_target_on_reach,
            ).chain())
        ;
    }
}

fn move_to_target(
    mut entities: Query<(&mut Transform, &TargetPosition, Option<&MovementSpeed>)>,
    time: Res<Time>,
) {
    for (mut transform, target, speed) in entities.iter_mut() {
        let target_position = target.0;
        let speed = speed.map_or(view::DEFAULT_MOVEMENT_SPEED, |s| s.0);

        let direction = (target_position - transform.translation).normalize();

        let distance_to_target = target_position.distance(transform.translation);
        let speed_factor = (distance_to_target / speed).clamp(0.0, 1.0);

        transform.translation += direction * speed * speed_factor * time.delta_seconds();

        if distance_to_target <= 0.1 {
            transform.translation = target_position;
        }
    }
}

fn remove_target_on_reach(
    mut commands: Commands,
    entities: Query<(Entity, &Transform, &TargetPosition)>,
) {
    for (entity, transform, target) in entities.iter() {
        let distance = target.0.distance(transform.translation);
        if distance <= 0.1 {
            commands.entity(entity).remove::<TargetPosition>();
        }
    }
}