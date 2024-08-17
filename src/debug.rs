#[cfg(debug_assertions)]
use bevy_editor_pls::EditorPlugin;
use crate::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                #[cfg(debug_assertions)]
                EditorPlugin::default(),
            ))
        ;
    }
}

