use bevy::winit::WinitSettings;
use crate::gameplay::GameplayPlugin;
use crate::infrastructure::InfrastructurePlugin;
use crate::prelude::*;
use crate::prelude::delayed_call::*;
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
            .insert_resource(WinitSettings::desktop_app())

            .add_plugins((
                ToolsPlugin,

                // --- 
                ViewPlugin,
                DelayedCallPlugin,
                InfrastructurePlugin,
                GameplayPlugin,
            ))
        ;
    }
}