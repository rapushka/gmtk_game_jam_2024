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
                    }),
                    DebugPlugin,
                ),
            )
        ;
    }
}
