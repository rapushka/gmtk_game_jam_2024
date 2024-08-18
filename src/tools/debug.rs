use bevy::ecs::observer::ObserverState;
#[cfg(debug_assertions)]
use bevy_editor_pls::EditorPlugin;
use crate::prelude::*;

type AppStateTransition = StateTransitionEvent<AppState>;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                #[cfg(debug_assertions)]
                EditorPlugin::default(),
            ))

            .add_systems(Update, log_state_transition.run_if(on_event::<AppStateTransition>()))

            .observe(rename_observers)
        ;
    }
}
fn log_state_transition(
    mut event: EventReader<AppStateTransition>,
) {
    for event in event.read() {
        info!("state transition: {:?} -> {:?}", event.exited, event.entered);
    }
}

fn rename_observers(
    trigger: Trigger<OnAdd, ObserverState>,
    mut commands: Commands,
) {
    commands.entity(trigger.entity())
        .insert(Name::new("observer"));
}