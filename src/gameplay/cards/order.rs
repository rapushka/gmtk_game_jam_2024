use crate::gameplay::cards::{Card, DeckRoot};
use crate::gameplay::cards::play_card::PlayPlayerCard;
use crate::prelude::*;

#[derive(Resource, Default)]
pub struct CardCount(u8);

#[derive(Component, Reflect)]
pub struct CardOrder(u8);

pub struct CardOrderingPlugin;

impl Plugin for CardOrderingPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<CardOrder>()

            .init_resource::<CardCount>()

            .observe(add_card_order)
            .observe(decrement_order_on_card_played)
        ;
    }
}

fn add_card_order(
    trigger: Trigger<OnAdd, Card>,
    mut commands: Commands,
    mut card_count: ResMut<CardCount>,
) {
    let card_entity = trigger.entity();
    let current_count = card_count.0;

    commands.entity(card_entity)
        .insert(CardOrder(current_count))
    ;

    card_count.0 += 1;
}

fn decrement_order_on_card_played(
    _trigger: Trigger<PlayPlayerCard>,
    mut cards: Query<&mut CardOrder>,
    card_count: Res<CardCount>,
) {
    for mut card_order in cards.iter_mut() {
        if card_order.0 == 0 {
            card_order.0 = card_count.0;
        }

        card_order.0 -= 1;
    }
}

// TODO: check out if it works
fn rearrange(
    mut deck_roots: Query<&mut Children, With<DeckRoot>>,
    cards: Query<&CardOrder>,
) {
    for mut children in deck_roots.iter_mut() {
        children.sort_by_key(|e| {
            let order = cards.get(*e).unwrap();
            order.0
        });
    }
}