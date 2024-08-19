use crate::gameplay::enemies::Enemy;
use crate::gameplay::game_loop::game_turn::GameTurn;
use crate::prelude::*;

#[derive(Component)]
pub struct IsMakingTurn(pub bool);

pub fn check_if_all_enemies_made_turn(
    enemies: Query<&IsMakingTurn, With<Enemy>>,
    prev_turn: Res<State<GameTurn>>,
    mut next_turn: ResMut<NextState<GameTurn>>,
) {
    let someone_is_making_turn = enemies.iter().any(|is_making_turn| is_making_turn.0);

    if !someone_is_making_turn {
        next_turn.set(GameTurn::Waiting {
            next_state: Box::new(prev_turn.flip()),
        });
    }
}
