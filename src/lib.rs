use crate::gameplay::GameplayPlugin;
use crate::infrastructure::InfrastructurePlugin;
use crate::prelude::*;
use crate::tools::ToolsPlugin;
use crate::view::ViewPlugin;

mod prelude;
mod player;
mod tools;
mod gameplay;
mod view;
mod infrastructure;
mod utils;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                ToolsPlugin,

                // --- 
                ViewPlugin,
                InfrastructurePlugin,
                GameplayPlugin,
            ))
            .add_systems(Startup, setup)
        ;
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("ducky.png"),
        ..Default::default()
    });
}