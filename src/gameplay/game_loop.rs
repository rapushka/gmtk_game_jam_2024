use crate::infrastructure::app_state::*;
use crate::prelude::*;

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(InGameplay = InGameplay)]
pub enum GamePhase {
    #[default]
    Setup,
    PlayerTurn,
    EnemyTurn,
    Encounter,
}

pub struct GameLoopPlugin;

impl Plugin for GameLoopPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_sub_state::<GamePhase>()

            .add_systems(OnEnter(AppState::in_gameplay()), start_with_player_turn)
        ;
    }
}

fn start_with_player_turn(
    mut next_state: ResMut<NextState<GamePhase>>
) {
    next_state.set(GamePhase::PlayerTurn);
}