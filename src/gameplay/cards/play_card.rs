use crate::gameplay::cards::order::*;
use crate::gameplay::cards::play_card::invoke::{InvokeCard, InvokeCardPlugin};
use crate::gameplay::cards::{Card, PlayerCard};
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
    characters: Query<(Entity, &TopCard, &Opponent), With<Character>>,
    cards: Query<&Card>,
) {
    for (character, top_card, enemy) in characters.iter() {
        let card = cards.get(top_card.0).expect("top card doesn't have CardType").0;

        commands.trigger(InvokeCard {
            card,
            sender: character,
            target: enemy.0,
        });
    }
}