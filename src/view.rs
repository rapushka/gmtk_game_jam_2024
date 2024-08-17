use crate::prelude::*;
use crate::view::ui::UiPlugin;
use self::camera::*;

mod camera;
pub mod ui;

pub struct ViewPlugin;

impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(UiPlugin)

            .add_systems(OnEnter(AppState::Bootstrap), spawn_camera)
        ;
    }
} 