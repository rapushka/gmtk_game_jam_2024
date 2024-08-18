use crate::gameplay::cards::play_card::PlayTopCard;
use crate::gameplay::game_loop::autoplay::*;
use crate::infrastructure::app_state::*;
use crate::prelude::*;
use autoplay::AutoplayState;
use game_turn::GameTurn;
use crate::gameplay::game_loop::game_turn::IsWaiting;
use crate::gameplay::game_loop::waiting::TurnWaitingPlugin;

pub mod autoplay;
mod waiting;
pub mod game_turn;

pub struct GameLoopPlugin;

impl Plugin for GameLoopPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<AutoplayState>()
            .add_computed_state::<IsAutoPlaying>()
            .add_sub_state::<GameTurn>()
            .add_computed_state::<IsWaiting>()

            .add_plugins(TurnWaitingPlugin)

            .add_systems(OnEnter(AppState::in_gameplay()), start_with_player_turn)
            .add_systems(Update, (
                play_top_card,
                reset_autoplay,
                pass_turn,
            ).chain()
                .run_if(in_state(AutoplayState::is_playing()))
                .run_if(in_state(GameTurn::PlayerTurn)),
            )

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

fn play_top_card(
    mut commands: Commands,
) {
    commands.trigger(PlayTopCard);
}

fn pass_turn(
    prev_turn: Res<State<GameTurn>>,
    mut next_turn: ResMut<NextState<GameTurn>>,
) {
    next_turn.set(GameTurn::Waiting {
        next_state: Box::new(prev_turn.flip()),
    });
}