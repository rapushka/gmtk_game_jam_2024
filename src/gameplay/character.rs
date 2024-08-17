use bevy::sprite::Anchor;
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
    assets: Res<CharacterAssets>,
) {
    let sprite_handle = assets.bouncer.clone();
    let character = commands.spawn_with_name("character_bouncer")
        .insert(SpriteBundle {
            texture: sprite_handle,
            sprite: Sprite {
                anchor: Anchor::BottomCenter,
                ..default()
            },
            ..default()
        })
        .id();

    // ---
    commands.add(SpawnRoundedRectCommand {
        name: "background",
        size: Vec2::new(300.0, 100.0),
        position: Vec2::ZERO,
        parent: Some(character),
        ..default()
    });
}