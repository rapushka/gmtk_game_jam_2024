use crate::gameplay::game_loop::autoplay::Autoplay;
use crate::prelude::*;
use crate::prelude::gameplay_hud::{AutoPlayButton, NextTurnButton};

pub fn on_next_turn_button_clicked(
    trigger: Trigger<Clicked>,
    query: Query<&NextTurnButton>,
    mut playmode: ResMut<Autoplay>,
) {
    let target = trigger.entity();
    if !query.contains(target) {
        return;
    }

    *playmode = Autoplay::Play { repeat: false };
}

pub fn on_autoplay_button_clicked(
    trigger: Trigger<Clicked>,
    query: Query<&AutoPlayButton>,
    mut playmode: ResMut<Autoplay>,
) {
    let target = trigger.entity();
    if !query.contains(target) {
        return;
    }

    *playmode = Autoplay::Play { repeat: true };
}