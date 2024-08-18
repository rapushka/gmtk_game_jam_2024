use crate::prelude::*;
use bevy::ecs::world::Command;
use bevy::prelude::Entity;
use crate::gameplay::cards::deck::Deck;

pub struct SetupDeck(pub Entity);

impl Command for SetupDeck {
    fn apply(self, world: &mut World) {
        let deck = world.get::<Deck>(self.0)
            .expect("SetupDeck must be called only on an entity with Deck component")
            .0.clone();

        let mut commands = world.commands();

        for card in deck {
            commands.add(SetupCard { entity: card });
        }
    }
}

pub struct SetupCard {
    pub entity: Entity,
}

impl Command for SetupCard {
    fn apply(self, world: &mut World) {
        todo!()
    }
}