use bevy::utils::info;
use crate::gameplay::game_loop::autoplay::*;
use autoplay::AutoplayState;
use crate::gameplay::cards::play_card::PlayTopCard;
use crate::infrastructure::app_state::*;
use crate::prelude::*;

pub mod autoplay;

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(InGameplay = InGameplay)]
pub enum GameTurn {
    #[default]
    Setup,
    PlayerTurn,
    EnemyTurn,
    Waiting {
        next_state: Box<GameTurn>,
    },
    Encounter,
}

impl GameTurn {
    pub fn flip(&self) -> Self {
        match *self {
            GameTurn::PlayerTurn => GameTurn::EnemyTurn,
            GameTurn::EnemyTurn => GameTurn::PlayerTurn,
            _ => panic!("can't flip other states!"),
        }
    }
}

pub struct GameLoopPlugin;

impl Plugin for GameLoopPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<AutoplayState>()
            .add_computed_state::<IsAutoPlaying>()
            .add_sub_state::<GameTurn>()

            .add_systems(OnEnter(AppState::in_gameplay()), start_with_player_turn)
            .add_systems(Update, (
                play_card,
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

fn play_card(
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