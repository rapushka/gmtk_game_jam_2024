use crate::gameplay::cards::Card;
use crate::gameplay::cards::setup::*;
use crate::gameplay::cards::types::CardType;
use crate::prelude::*;

#[derive(Event)]
pub struct SpawnDeck {
    pub initial_cards: Vec<CardType>,
    pub position: Vec3,
    pub parent: Entity,
}

#[derive(Component)]
pub struct Deck(pub Vec<Entity>);

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

    let deck = commands.spawn_with_name("deck")
        .set_parent(event.parent)
        .insert(Transform::from_translation(event.position))
        .with_children(|deck| {
            for card_type in card_types {
                let card_entity = deck
                    .spawn_with_name(&format!("card: {}", card_type.name()))
                    .insert(Card(*card_type))
                    .id();

                card_entities.push(card_entity);
            }
        })

        .insert(Deck(card_entities))
        .id();

    commands.add(SetupDeck(deck));
}