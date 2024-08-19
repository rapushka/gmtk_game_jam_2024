use crate::gameplay::cards::deck::{DeckPlugin, SpawnDeck};
use crate::gameplay::cards::order::CardOrderingPlugin;
use crate::gameplay::cards::play_card::*;
use crate::gameplay::cards::types::CardType;
use crate::gameplay::character::Character;
use crate::prelude::*;
use crate::view::ui::gameplay_hud::spawn::spawn_gameplay_hud;

pub mod deck;
pub mod types;
pub mod setup;
pub mod order;
pub mod play_card;

#[derive(Component)]
pub struct Card(pub CardType);

pub struct CardsPlugin;

impl Plugin for CardsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                CardOrderingPlugin,
                PlayCardPlugin,
                DeckPlugin,
            ))

            .add_systems(OnEnter(AppState::in_gameplay()), test_cards_spawn.after(spawn_gameplay_hud))
        ;
    }
}

fn test_cards_spawn(
    mut commands: Commands,
    characters: Query<Entity, With<Character>>,
) {
    let attack = CardType::Attack(balance::ATTACK_COEFFICIENT);

    for character in characters.iter() {
        commands.trigger(SpawnDeck {
            initial_cards: vec![attack; 5],
            position: Vec2::new(465.0, -200.0).extend(1.0),
            parent: None,
            owner: character,
        });
    }
}