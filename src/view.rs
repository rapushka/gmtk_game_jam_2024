use crate::prelude::*;
use crate::view::move_to_target::MoveToTargetPlugin;
use crate::view::ui::UiPlugin;
use self::camera::*;

mod camera;
pub mod ui;
pub mod move_to_target;

pub struct ViewPlugin;

impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                UiPlugin,
                MoveToTargetPlugin,
            ))

            .add_systems(OnEnter(AppState::Bootstrap), spawn_camera)
        ;
    }
} 