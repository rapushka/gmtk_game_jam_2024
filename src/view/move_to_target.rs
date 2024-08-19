use crate::prelude::*;

#[derive(Component)]
pub struct TargetPosition(pub Vec3);
#[derive(Component)]
pub struct MovementSpeed(pub f32);

pub struct MoveToTargetPlugin;

impl Plugin for MoveToTargetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, move_to_target)
        ;
    }
}

fn move_to_target(
    mut commands: Commands,
    mut entities: Query<(Entity, &mut Transform, &TargetPosition, Option<&MovementSpeed>)>,
    time: Res<Time>,
) {
    for (entity, mut transform, target, speed) in entities.iter_mut() {
        let position = &mut transform.translation;
        let target = target.0;
        let speed = speed.map_or(view::DEFAULT_MOVEMENT_SPEED, |s| s.0);

        let direction = (target - *position).normalize();
        let distance = target.distance(*position);
        let delta = speed * time.delta_seconds();

        if delta >= distance {
            *position = target;
            commands.entity(entity).remove::<TargetPosition>();

            continue;
        }

        *position += direction * delta;
    }
}