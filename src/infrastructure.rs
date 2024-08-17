use crate::prelude::*;

pub mod app_state;

pub struct InfrastructurePlugin;

impl Plugin for InfrastructurePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<AppState>()

            .add_systems(OnEnter(AppState::Bootstrap), skip_to_gameplay)
        ;
    }
}

fn skip_to_gameplay(
    mut app_state: ResMut<NextState<AppState>>,
) {
    app_state.set(AppState::start_gameplay())
}