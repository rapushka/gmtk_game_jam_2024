use crate::gameplay::cards::spawn::SpawnCard;
use crate::prelude::*;
use crate::gameplay::enemies::Enemy;

pub fn spawn_enemy_cards(
    trigger: Trigger<OnAdd, Enemy>,
    mut commands: Commands,
    enemies: Query<&Enemy>,
) {
    let enemy_entity = trigger.entity();
    let enemy_type = enemies.get(enemy_entity).unwrap().0;

    for card_type in enemy_type.cards() {
        commands.trigger(SpawnCard {
            card_type,
            parent: enemy_entity,
            is_player_card: false,
        });
    }
}