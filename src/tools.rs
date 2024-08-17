use bevy::asset::AssetMetaCheck;
use crate::prelude::*;
use self::debug::DebugPlugin;

mod debug;

pub struct ToolsPlugin;

impl Plugin for ToolsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(
                (
                    DefaultPlugins.set(AssetPlugin {
                        meta_check: AssetMetaCheck::Never,
                        ..default()
                    }).set(WindowPlugin {
                        primary_window: Some(Window {
                            title: "Deck-Sizer".to_string(),
                            resolution: (1280.0, 720.0).into(),
                            resizable: false,
                            ..default()
                        }),
                        ..default()
                    }),
                    DebugPlugin,
                ),
            )
        ;
    }
}
