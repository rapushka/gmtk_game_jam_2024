use crate::prelude::{App, Plugin};
use crate::view::ui::gameplay_hud::GameplayUiPlugin;

pub mod gameplay_hud;
pub mod create;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(GameplayUiPlugin)
        ;
    }
}

