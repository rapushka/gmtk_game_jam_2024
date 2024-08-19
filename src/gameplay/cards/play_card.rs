use crate::gameplay::cards::deck::unit_ownership::OwnedDeck;
use crate::gameplay::cards::deck::deck_component::Deck;
use crate::gameplay::cards::play_card::invoke::{InvokeCard, InvokeCardPlugin};
use crate::gameplay::cards::*;
use crate::gameplay::character::Character;
use crate::gameplay::game_loop::game_turn::GameTurn;
use crate::prelude::*;

mod invoke;

#[derive(Event)]
pub struct PlayTopCard; // i.e. the next card, in player's hand it's the button card tho

#[derive(Event)]
pub struct PlayPlayerCard;
#[derive(Event)]
pub struct PlayEnemyCard;

pub struct PlayCardPlugin;

impl Plugin for PlayCardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlayTopCard>()
            .add_event::<PlayPlayerCard>()
            .add_event::<PlayEnemyCard>()

            .add_plugins(InvokeCardPlugin)

            .observe(on_play_top_card)
            .observe(on_enemy_play_card)
            .observe(on_player_play_card)
        ;
    }
}

fn on_play_top_card(
    _trigger: Trigger<PlayTopCard>,
    current_turn: Res<State<GameTurn>>,
    mut commands: Commands,
) {
    match **current_turn {
        GameTurn::PlayerTurn => commands.trigger(PlayPlayerCard),
        GameTurn::EnemyTurn => commands.trigger(PlayEnemyCard),
        _ => panic!("you can't play card from other states!")
    }
}

fn on_enemy_play_card(
    _trigger: Trigger<PlayEnemyCard>,
) {
    todo!();
}

fn on_player_play_card(
    _trigger: Trigger<PlayPlayerCard>,
    mut commands: Commands,
    characters: Query<(Entity, &Opponent, &OwnedDeck), With<Character>>,
    mut decks: Query<&mut Deck>,
    cards: Query<&Card>,
) {
    for (character, enemy, owned_deck) in characters.iter() {
        let mut deck = decks.get_mut(owned_deck.0).expect("character can't own not-deck in OwnedDeck");
        let top_card = deck.pick_top_card().expect("TODO: handle case if the deck is empty");
        let card = cards.get(top_card).expect("top card can't be a not-card!");

        commands.trigger(InvokeCard {
            card: card.0,
            sender: character,
            target: enemy.0,
        });
    }
}