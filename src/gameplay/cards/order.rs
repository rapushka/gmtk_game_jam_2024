#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use crate::gameplay::cards::*;
use crate::gameplay::cards::play_card::PlayPlayerCard;
use crate::gameplay::character::Character;
use crate::prelude::*;

#[derive(Component, Reflect)]
pub struct CardOrder(pub u8);

pub struct CardOrderingPlugin;

impl Plugin for CardOrderingPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<CardOrder>()

            .observe(decrement_order_on_card_played)
        ;
    }
}

fn decrement_order_on_card_played(
    _trigger: Trigger<PlayPlayerCard>,
    mut cards: Query<&mut CardOrder>,
) {
    // TODO
}