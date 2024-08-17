use crate::prelude::*;
use crate::view::ui::gameplay::spawn::spawn_gameplay_hud;

mod spawn;

#[derive(Component)]
pub struct AutoPlayButton;

pub struct GameplayUiPlugin;

impl Plugin for GameplayUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::in_gameplay()), spawn_gameplay_hud)
        ;
    }
}