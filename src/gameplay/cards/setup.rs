use crate::prelude::*;
use bevy::ecs::world::Command;
use bevy::prelude::Entity;
use crate::gameplay::cards::Card;
use crate::gameplay::cards::deck::Deck;
use crate::prelude::spawn::rounded_square::SpawnRoundedRectCommand;
use crate::utils::spawn::text::SpawnTextCommand;

pub struct SetupDeck(pub Entity);

impl Command for SetupDeck {
    fn apply(self, world: &mut World) {
        let deck = world.get::<Deck>(self.0)
            .expect("SetupDeck must be called only on an entity with Deck component")
            .0.clone();

        let mut commands = world.commands();

        for card in deck {
            commands.add(SetupCardView { entity: card });
        }
    }
}

pub struct SetupCardView {
    pub entity: Entity,
}

impl Command for SetupCardView {
    fn apply(self, world: &mut World) {
        let card_entity = self.entity;
        let card_type = world.get::<Card>(card_entity)
            .expect("SetupCard must only be called on Cards").0;

        let mut commands = world.commands();
        let background = commands.spawn_with_name("background").id();

        commands.add(SpawnRoundedRectCommand {
            entity: Some(background),
            name: "background",
            size: view::CARD_ITEM_SIZE,
            is_under_parent: true,
            color: colors::card_background_color(),
            parent: Some(card_entity),
            ..default()
        });

        commands.add(SpawnTextCommand {
            text: card_type.name(),
            parent: Some(background),
            ..default()
        });
    }
}