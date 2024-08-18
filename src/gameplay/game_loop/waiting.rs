use std::time::Duration;
use crate::gameplay::game_loop::game_turn::GameTurn;
use crate::prelude::*;

#[derive(Resource)]
struct WaitingTimer(Timer);

pub struct TurnWaitingPlugin;

impl Plugin for TurnWaitingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameTurn::is_waiting()), start_timer)
            .add_systems(Update, tick_timer.run_if(in_state(GameTurn::is_waiting())))

            .add_systems(OnExit(GameTurn::is_waiting()), remove_timer)
        ;
    }
}

fn start_timer(
    mut commands: Commands,
) {
    let duration = Duration::from_secs_f32(view::timings::TURN_PASS_DELAY);
    let timer = Timer::new(duration, TimerMode::Once);
    commands.insert_resource(WaitingTimer(timer));
}

fn tick_timer(
    mut waiting_timer: ResMut<WaitingTimer>,
    time: Res<Time>,
    prev_turn: Res<State<GameTurn>>,
    mut next_turn: ResMut<NextState<GameTurn>>,
) {
    waiting_timer.0.tick(time.delta());

    if waiting_timer.0.finished() {
        if let GameTurn::Waiting { next_state } = (**prev_turn).clone() {
            next_turn.set(*next_state);
        }
    }
}

fn remove_timer(
    mut commands: Commands,
) {
    commands.remove_resource::<WaitingTimer>();
}