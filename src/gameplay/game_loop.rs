use crate::gameplay::game_loop::autoplay::*;
use autoplay::AutoplayState;
use crate::infrastructure::app_state::*;
use crate::prelude::*;

mod autoplay;

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
            .init_state::<AutoplayState>()
            .add_sub_state::<GamePhase>()

            .add_systems(OnEnter(AppState::in_gameplay()), start_with_player_turn)
            .add_systems(OnEnter(GamePhase::PlayerTurn), reset_autoplay.run_if(in_state(AutoplayState::is_playing())))

            .observe(on_next_turn_button_clicked)
            .observe(on_autoplay_button_clicked)
        ;
    }
}

fn start_with_player_turn(
    mut next_state: ResMut<NextState<GamePhase>>
) {
    next_state.set(GamePhase::PlayerTurn);
}