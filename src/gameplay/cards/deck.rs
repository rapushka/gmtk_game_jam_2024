use crate::gameplay::cards::order::CardOrder;
use crate::gameplay::cards::setup::*;
use crate::gameplay::cards::types::CardType;
use crate::gameplay::cards::Card;
use crate::gameplay::cards::deck::unit_ownership::OwnedDeck;
use crate::prelude::*;

pub mod unit_ownership;

#[derive(Event)]
pub struct SpawnDeck {
    pub initial_cards: Vec<CardType>,
    pub position: Vec3,
    pub parent: Option<Entity>,
    pub owner: Entity,
}

#[derive(Component)]
pub struct Deck {
    pub cards: Vec<Entity>,
    pub top_card: Option<Entity>,
    pub cards_counter: u8,
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
    let mut card_entities = Vec::new();

    let mut tmp = commands.spawn_with_name("deck");
    let mut top_card = None;
    let mut cards_counter = 0;

    let entity_command = tmp
        .insert(GlobalTransform::default())
        .insert(InheritedVisibility::default())
        .insert(Transform::from_translation(event.position))
        .with_children(|deck| {
            for card_type in card_types {
                let card_entity = deck
                    .spawn_with_name(&format!("card: {}", card_type.name()))
                    .insert(GlobalTransform::default())
                    .insert(InheritedVisibility::default())
                    .insert(Transform::default())
                    .insert(Card(*card_type))
                    .insert(CardOrder(cards_counter))
                    .id();

                card_entities.push(card_entity);

                if top_card == None {
                    top_card = Some(card_entity);
                }

                cards_counter += 1;
            }
        })

        .insert(Deck {
            cards: card_entities,
            top_card,
            cards_counter,
        });
    let deck = entity_command.id();

    if let Some(parent) = event.parent {
        entity_command.set_parent(parent);
    }

    commands.entity(event.owner).insert(OwnedDeck(deck));

    commands.add(SetupDeck(deck));
}