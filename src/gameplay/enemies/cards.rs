use bevy::ecs::query::QueryEntityError;
use crate::gameplay::cards::deck::*;
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

    commands.trigger(SpawnDeck {
        initial_cards: enemy.0.cards(),
        position: Vec3::new(0.0, 250.0, 1.0),
        parent: Some(card_holder.0),
    });
}