use crate::prelude::*;

pub fn spawn_camera(
    mut commands: Commands
) {
    commands.spawn_with_name("camera")
        .insert(Camera2dBundle::default())
        .insert(IsDefaultUiCamera);
}