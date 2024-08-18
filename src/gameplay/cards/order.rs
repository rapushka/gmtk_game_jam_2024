#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use crate::gameplay::cards::{Card, DeckRoot, PlayerCard};
use crate::gameplay::cards::play_card::PlayPlayerCard;
use crate::gameplay::character::Character;
use crate::prelude::*;

#[derive(Resource, Default)]
pub struct CardCount(u8);

#[derive(Component, Reflect)]
pub struct CardOrder(u8);

#[derive(Component)]
pub struct TopCard(pub Entity);

pub struct CardOrderingPlugin;

impl Plugin for CardOrderingPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<CardOrder>()

            .init_resource::<CardCount>()

            .add_systems(Update, update_player_top_card)

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

fn update_player_top_card(
    mut commands: Commands,
    characters: Query<Entity, With<Character>>,
    cards: Query<(Entity, &CardOrder), (With<PlayerCard>, Changed<CardOrder>)>,
) {
    for (card, order) in cards.iter() {
        if order.0 != 0 {
            continue;
        }

        for character in characters.iter() {
            commands.entity(character).insert(TopCard(card));
        }
    }
}