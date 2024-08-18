use std::time::Duration;
use crate::prelude::*;

#[derive(Component)]
pub struct DelayedEvent<E: Event> {
    timer: Timer,
    event: Option<E>,
}

impl<E: Event> DelayedEvent<E> {
    pub fn new(delay: f32, event: E) -> Self {
        Self {
            timer: Timer::new(Duration::from_secs_f32(delay), TimerMode::Once),
            event: Some(event),
        }
    }
}

pub(super) struct DelayedCallPlugin;

impl Plugin for DelayedCallPlugin {
    fn build(&self, app: &mut App) {
        app
        // TODO: add timers
        // .add_systems(Update, tick_timer::<>)
        ;
    }
}

fn tick_timer<E: Event>(
    mut commands: Commands,
    mut delayed_events: Query<(Entity, &mut DelayedEvent<E>)>,
    time: Res<Time>,
) {
    for (entity, mut delayed_event) in delayed_events.iter_mut() {
        delayed_event.timer.tick(time.delta());

        if delayed_event.timer.finished() {
            let event = delayed_event.event.take().expect("who took the event?:(");
            commands.trigger(event);
            commands.entity(entity).despawn_recursive();
        }
    }
}