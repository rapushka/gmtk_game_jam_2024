use crate::gameplay::game_loop::game_turn::GameTurn;
use crate::prelude::*;

pub struct EnemyAiPlugin;

impl Plugin for EnemyAiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameTurn::EnemyTurn), start_picking_card)
        ;
    }
}

fn start_picking_card() {
    info!("--- aiii");
}