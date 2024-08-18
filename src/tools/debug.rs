use std::any::type_name;
use bevy::ecs::observer::ObserverState;
#[cfg(debug_assertions)]
use bevy_editor_pls::EditorPlugin;
use crate::gameplay::game_loop::autoplay::AutoplayState;
use crate::gameplay::game_loop::game_turn::GameTurn;
use crate::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                #[cfg(debug_assertions)]
                EditorPlugin::default(),
            ))

            .add_systems(Update, log_state_transition::<AppState>)
            .add_systems(Update, log_state_transition::<GameTurn>)
            .add_systems(Update, log_state_transition::<AutoplayState>)

            .observe(rename_observers)
        ;
    }
}
fn log_state_transition<T: States>(
    mut event: EventReader<StateTransitionEvent<T>>,
) {
    for event in event.read() {
        let state = get_short_type_name::<T>();
        info!("[{}] state transition: {:?} -> {:?}", state, event.exited, event.entered);
    }
}

fn get_short_type_name<T: ?Sized>() -> &'static str {
    let full_name = type_name::<T>();
    full_name.split("::").last().unwrap_or(full_name)
}

fn rename_observers(
    trigger: Trigger<OnAdd, ObserverState>,
    mut commands: Commands,
) {
    commands.entity(trigger.entity())
        .insert(Name::new("observer"));
}