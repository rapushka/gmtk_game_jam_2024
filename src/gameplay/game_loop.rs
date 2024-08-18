use crate::gameplay::game_loop::autoplay::*;
use autoplay::AutoplayState;
use crate::gameplay::cards::play_card::PlayTopCard;
use crate::infrastructure::app_state::*;
use crate::prelude::*;

mod autoplay;

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(InGameplay = InGameplay)]
pub enum GameTurn {
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
            .add_sub_state::<GameTurn>()

            .add_systems(OnEnter(AppState::in_gameplay()), start_with_player_turn)
            .add_systems(OnEnter(GameTurn::PlayerTurn), (
                play_card,
                reset_autoplay,
            ).chain()
                .run_if(in_state(AutoplayState::is_playing())))

            .observe(on_next_turn_button_clicked)
            .observe(on_autoplay_button_clicked)
        ;
    }
}

fn start_with_player_turn(
    mut next_state: ResMut<NextState<GameTurn>>
) {
    next_state.set(GameTurn::PlayerTurn);
}

fn play_card(
    mut commands: Commands,
) {
    commands.trigger(PlayTopCard);
}