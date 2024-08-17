use crate::prelude::*;
use crate::prelude::spawn::rounded_square::SpawnRoundedRectCommand;

pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<TextureSlicer>()

            .add_systems(OnEnter(AppState::in_gameplay()), spawn_character)
        ;
    }
}

fn spawn_character(
    mut commands: Commands,
) {
    commands.add(SpawnRoundedRectCommand {
        name: "background",
        size: Vec2::new(300.0, 100.0),
        position: Vec2::ZERO,
        ..default()
    });
}