use crate::gameplay::game_loop::autoplay::AutoplayState;
use crate::gameplay::game_loop::GameTurn;
use crate::prelude::*;
use crate::prelude::gameplay_hud::{AutoPlayButton, NextTurnButton};

pub fn on_next_turn_button_clicked(
    trigger: Trigger<Clicked>,
    query: Query<&NextTurnButton>,
    mut playmode: ResMut<NextState<AutoplayState>>,
    game_turn: Res<State<GameTurn>>,
) {
    if **game_turn != GameTurn::PlayerTurn {
        return;
    }

    let target = trigger.entity();
    if !query.contains(target) {
        return;
    }

    playmode.set(AutoplayState::Play { repeat: false });
}

pub fn on_autoplay_button_clicked(
    trigger: Trigger<Clicked>,
    query: Query<&AutoPlayButton>,
    mut playmode: ResMut<NextState<AutoplayState>>,
) {
    let target = trigger.entity();
    if !query.contains(target) {
        return;
    }

    playmode.set(AutoplayState::Play { repeat: true });
}