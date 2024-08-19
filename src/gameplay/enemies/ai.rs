use crate::gameplay::enemies::Enemy;
use crate::gameplay::game_loop::game_turn::GameTurn;
use crate::prelude::*;
use crate::prelude::delayed_call::DelayedEvent;

#[derive(Event)]
pub struct EnemyThinking(Entity);

pub struct EnemyAiPlugin;

impl Plugin for EnemyAiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameTurn::EnemyTurn), start_picking_card)

            .observe(on_enemy_thinking_ended)
        ;
    }
}

fn start_picking_card(
    mut commands: Commands,
    enemies: Query<Entity, With<Enemy>>,
) {
    for enemy in enemies.iter() {
        let event = EnemyThinking(enemy);
        let delay = view::timings::ENEMY_THINKING_SECONDS;

        commands
            .spawn_with_name("enemy thinking timer")
            .insert(DelayedEvent::new(delay, event))
        ;
    }
}

fn on_enemy_thinking_ended(
    trigger: Trigger<EnemyThinking>,
) {
    let target = trigger.event().0;
}