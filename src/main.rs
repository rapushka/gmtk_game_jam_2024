#![windows_subsystem = "windows"]

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use crate::debug::DebugPlugin;

mod prelude;
mod player;

fn main() {
    App::new()
        .add_plugins(DebugPlugin)
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            meta_check: AssetMetaCheck::Never,
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("ducky.png"),
        ..Default::default()
    });
}
