use crate::gameplay::cards::CardsPlugin;
use crate::gameplay::character::CharactersPlugin;
use crate::gameplay::enemies::EnemiesPlugin;
use crate::gameplay::game_loop::GameLoopPlugin;
use crate::gameplay::health::HealthPlugin;
use crate::prelude::*;

mod character;
mod enemies;
pub mod cards;
pub mod health;
pub mod unit_common;
mod game_loop;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                GameLoopPlugin,
                CharactersPlugin,
                EnemiesPlugin,
                CardsPlugin,
                HealthPlugin,
            ))
        ;
    }
}