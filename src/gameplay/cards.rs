use crate::gameplay::cards::deck::{DeckPlugin, SpawnDeck};
use crate::gameplay::cards::order::CardOrderingPlugin;
use crate::gameplay::cards::play_card::*;
use crate::gameplay::cards::types::CardType;
use crate::prelude::*;
use crate::view::ui::gameplay_hud::spawn::spawn_gameplay_hud;

pub mod deck;
pub mod types;
pub mod setup;
pub mod order;
pub mod play_card;

#[derive(Component)]
pub struct DeckRoot;

#[derive(Component)]
pub struct Card(pub CardType);

#[derive(Component)]
pub struct PlayerCard;

#[derive(Component)]
pub struct EnemyCard;

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
    deck_root: Query<Entity, With<DeckRoot>>,
) {
    let attack = CardType::Attack(balance::ATTACK_COEFFICIENT);
    let deck_root = deck_root.get_single().unwrap();

    commands.trigger(SpawnDeck {
        initial_cards: vec![attack; 5],
        position: Vec2::new(0.0, 0.0).extend(1.0), // TODO: padličyć pazicyju z UI,
        parent: deck_root,
    });
}