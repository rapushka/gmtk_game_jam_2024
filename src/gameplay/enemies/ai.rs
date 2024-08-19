use crate::gameplay::cards::deck::unit_ownership::OwnedDeck;
use crate::gameplay::cards::play_card::PlayTopCard;
use crate::gameplay::enemies::ai::turn_passing::{check_if_all_enemies_made_turn, IsMakingTurn};
use crate::gameplay::enemies::Enemy;
use crate::gameplay::game_loop::game_turn::GameTurn;
use crate::prelude::delayed_call::DelayedEvent;
use crate::prelude::*;

mod turn_passing;

#[derive(Event)]
pub struct EnemyThinking(Entity);

pub struct EnemyAiPlugin;

impl Plugin for EnemyAiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameTurn::EnemyTurn), start_picking_card)

            .add_systems(Update, check_if_all_enemies_made_turn.after(start_picking_card)
                .run_if(in_state(GameTurn::EnemyTurn)))

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

        commands.entity(enemy).insert(IsMakingTurn(true));
    }
}

fn on_enemy_thinking_ended(
    trigger: Trigger<EnemyThinking>,
    mut commands: Commands,
    enemies: Query<(Entity, &Opponent, &OwnedDeck), With<Enemy>>,
) {
    let target = trigger.event().0;
    let (entity, opponent, deck) = enemies.get(target).expect("must be enemy!");

    commands.trigger(PlayTopCard {
        sender: entity,
        target: opponent.0,
        deck: deck.0,
    });

    commands.entity(entity).insert(IsMakingTurn(false));
}