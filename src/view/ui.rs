use crate::prelude::*;
use crate::view::ui::common::{click_on_pressed_button, visualise_interaction_with_buttons};
use crate::view::ui::gameplay_hud::GameplayUiPlugin;

pub use self::common::*;

pub mod gameplay_hud;
pub mod create;
mod common;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(GameplayUiPlugin)

            .add_systems(Update, (
                visualise_interaction_with_buttons,
                click_on_pressed_button,
            ))
        ;
    }
}

