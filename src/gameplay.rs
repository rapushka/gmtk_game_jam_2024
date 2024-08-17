use crate::gameplay::character::CharactersPlugin;
use crate::prelude::*;

mod character;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(CharactersPlugin)
        ;
    }
}