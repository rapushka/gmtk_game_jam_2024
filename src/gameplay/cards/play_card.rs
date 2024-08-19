use crate::gameplay::cards::deck::unit_ownership::OwnedDeck;
use crate::gameplay::cards::deck::deck_component::Deck;
use crate::gameplay::cards::play_card::invoke::{InvokeCard, InvokeCardPlugin};
use crate::gameplay::cards::*;
use crate::gameplay::character::Character;
use crate::gameplay::game_loop::game_turn::GameTurn;
use crate::prelude::*;

mod invoke;

#[derive(Event)]
pub struct PlayTopPlayerCard; // i.e. the next card, in player's hand it's the button card tho

#[derive(Event)]
pub struct PlayTopCard {
    pub deck: Entity,
    pub sender: Entity,
    pub target: Entity,
}

pub struct PlayCardPlugin;

impl Plugin for PlayCardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlayTopPlayerCard>()
            .add_event::<PlayTopCard>()

            .add_plugins(InvokeCardPlugin)

            .observe(on_play_player_top_card)
            .observe(trigger_play_top_card)
        ;
    }
}

fn on_play_player_top_card(
    _trigger: Trigger<PlayTopPlayerCard>,
    mut commands: Commands,
    current_turn: Res<State<GameTurn>>,
    characters: Query<(Entity, &Opponent, &OwnedDeck), With<Character>>,
) {
    if **current_turn != GameTurn::PlayerTurn {
        panic!("this event is only for player's turn");
    }

    for (character, opponent, owned_deck) in characters.iter() {
        commands.trigger(PlayTopCard {
            sender: character,
            target: opponent.0,
            deck: owned_deck.0,
        });
    }
}

fn trigger_play_top_card(
    trigger: Trigger<PlayTopCard>,
    mut commands: Commands,
    mut decks: Query<&mut Deck>,
    cards: Query<&Card>,
) {
    let PlayTopCard { deck, sender, target, } = trigger.event();

    let mut deck = decks.get_mut(*deck).expect("OwnedDeck must contain the Deck");
    let top_card = deck.pick_top_card().expect("TODO: handle case if the deck is empty");
    let card = cards.get(top_card).expect("top card can't be a not-card!");

    commands.trigger(InvokeCard {
        card: card.0,
        sender: *sender,
        target: *target,
    });
}
