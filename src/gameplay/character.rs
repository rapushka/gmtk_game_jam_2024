use crate::prelude::*;

pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::in_gameplay()), spawn_character)
        ;
    }
}

fn spawn_character(
    mut commands: Commands,
    assets: Res<CommonAssets>,
) {
    commands.spawn(SpriteBundle {
        texture: assets.rounded_square.clone(),
        ..Default::default()
    });
}