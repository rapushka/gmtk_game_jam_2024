use crate::gameplay::cards::deck::*;
use crate::gameplay::enemies::*;
use crate::prelude::*;

pub fn spawn_enemy_deck(
    trigger: Trigger<OnAdd, Enemy>,
    mut commands: Commands,
    enemies: Query<&Enemy>,
) {
    let entity = trigger.entity();
    let enemy = enemies.get(entity).unwrap();

    commands.trigger(SpawnDeck {
        initial_cards: enemy.0.cards(),
        position: Vec3::new(0.0, 250.0, 1.0),
        parent: Some(entity),
        owner: entity,
    });
}