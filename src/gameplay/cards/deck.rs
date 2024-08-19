use crate::gameplay::cards::order::CardOrder;
use crate::gameplay::cards::setup::*;
use crate::gameplay::cards::types::CardType;
use crate::gameplay::cards::Card;
use crate::gameplay::cards::deck::unit_ownership::OwnedDeck;
use crate::prelude::*;
pub use deck_component::*;

pub mod unit_ownership;
pub mod deck_component;

#[derive(Event)]
pub struct SpawnDeck {
    pub initial_cards: Vec<CardType>,
    pub position: Vec3,
    pub parent: Option<Entity>,
    pub owner: Entity,
}

pub struct DeckPlugin;

impl Plugin for DeckPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnDeck>()

            .observe(spawn_deck)
        ;
    }
}

fn spawn_deck(
    trigger: Trigger<SpawnDeck>,
    mut commands: Commands,
) {
    let event = trigger.event();
    let card_types = event.initial_cards.iter();

    let mut tmp = commands.spawn_with_name("deck");

    let mut deck = Deck::new();
    let entity_command = tmp
        .insert(GlobalTransform::default())
        .insert(InheritedVisibility::default())
        .insert(Transform::from_translation(event.position))
        .with_children(|parent| {
            for card_type in card_types {
                let card_entity = parent
                    .spawn_with_name(&format!("card: {}", card_type.name()))
                    .insert(GlobalTransform::default())
                    .insert(InheritedVisibility::default())
                    .insert(Transform::default())
                    .insert(Card(*card_type))
                    .insert(CardOrder(deck.card_count()))
                    .id();

                deck.add_card(card_entity);
            }
        })

        .insert(deck);
    let deck = entity_command.id();

    if let Some(parent) = event.parent {
        entity_command.set_parent(parent);
    }

    commands.entity(event.owner).insert(OwnedDeck(deck));

    commands.add(SetupDeck(deck));
}