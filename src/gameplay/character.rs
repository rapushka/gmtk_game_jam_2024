use crate::prelude::*;

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
    assets: Res<CommonAssets>,
) {
    let sprite_handle = assets.rounded_square.clone();

    commands.spawn_with_name("background")
        .insert(SpriteBundle {
            texture: sprite_handle,
            sprite: Sprite {
                custom_size: Some(Vec2::new(300.0, 100.0)),
                ..default()
            },
            ..default()
        })
        .insert(ImageScaleMode::Sliced(TextureSlicer {
            border: BorderRect::square(15.0),
            center_scale_mode: SliceScaleMode::Tile { stretch_value: 0.1 },
            sides_scale_mode: SliceScaleMode::Tile { stretch_value: 0.2 },
            max_corner_scale: 1.0,
        }))
    ;
}