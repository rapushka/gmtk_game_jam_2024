use crate::gameplay::cards::CardsPlugin;
use crate::gameplay::character::CharactersPlugin;
use crate::gameplay::enemies::EnemiesPlugin;
use crate::prelude::*;

mod character;
mod enemies;
mod cards;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                CharactersPlugin,
                EnemiesPlugin,
                CardsPlugin,
            ))
        ;
    }
}