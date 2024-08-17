use crate::infrastructure::app_state::InGameplay;
use crate::infrastructure::assets::AssetLoadingPlugin;
use crate::prelude::*;

pub mod app_state;
pub mod assets;

pub struct InfrastructurePlugin;

impl Plugin for InfrastructurePlugin {
    fn build(&self, app: &mut App) {
        app
            .enable_state_scoped_entities::<AppState>()
            .enable_state_scoped_entities::<InGameplay>()
            .init_state::<AppState>()
            .add_computed_state::<InGameplay>()

            .add_plugins(AssetLoadingPlugin)
            
            .add_systems(OnEnter(AppState::MainMenu), skip_to_gameplay)
        ;
    }
}

fn skip_to_gameplay(
    mut app_state: ResMut<NextState<AppState>>,
) {
    app_state.set(AppState::start_gameplay())
}