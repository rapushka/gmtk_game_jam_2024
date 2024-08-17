use crate::prelude::*;
use self::camera::*;

mod camera;

pub struct ViewPlugin;

impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Bootstrap), spawn_camera)
        ;
    }
} 