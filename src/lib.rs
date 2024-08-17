use crate::debug::DebugPlugin;
use crate::prelude::*;
use crate::tools::ToolsPlugin;

mod debug;
mod prelude;
mod player;
mod tools;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                #[cfg(debug_assertions)] {
                    DebugPlugin
                },
                ToolsPlugin,
            ))
            .add_systems(Startup, setup)
        ;
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("ducky.png"),
        ..Default::default()
    });
}

