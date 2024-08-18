use bevy::ecs::query::QueryEntityError;
use crate::gameplay::cards::spawn::SpawnCard;
use crate::prelude::*;
use crate::gameplay::enemies::{CardsHolder, Enemy};

pub fn spawn_enemy_cards(
    trigger: Trigger<OnAdd, CardsHolder>,
    mut commands: Commands,
    enemies: Query<(&Enemy, &CardsHolder)>,
) {
    let enemy_entity = trigger.entity();
    info!("--- check");
    let result = enemies.get(enemy_entity);
    match result {
        Ok(_) => {}
        Err(err) => {
            println!("{}", err);
        }
    };
    let (enemy, card_holder) = result.unwrap();

    for card_type in enemy.0.cards() {
        commands.trigger(SpawnCard {
            card_type,
            parent: card_holder.0,
            is_player_card: false,
        });
    }
}