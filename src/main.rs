#![windows_subsystem = "windows"]
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::prelude::*;
use gmtk_game_jam_2024::GamePlugin;

fn main() {
    App::new().add_plugins(GamePlugin).run();
}
