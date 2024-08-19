use crate::gameplay::cards::deck::Deck;
use crate::gameplay::cards::deck::reordering::view::order_views;
use crate::prelude::*;

mod view;

#[derive(Component, Reflect)]
pub struct CardOrder(pub u8);

#[derive(Event)]
pub struct OnDeckReordered(pub Entity);

pub struct DeckOrderingPlugin;

impl Plugin for DeckOrderingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<OnDeckReordered>()

            .register_type::<CardOrder>()

            .add_systems(Update, reorder_deck_on_change)
            .add_systems(Update, order_views)
        ;
    }
}

fn reorder_deck_on_change(
    mut commands: Commands,
    decks: Query<&Deck, Changed<Deck>>,
) {
    for deck in decks.iter() {
        for (index, card) in deck.iter_cards() {
            commands.entity(*card).insert(CardOrder(index as u8));
        }
    }
}