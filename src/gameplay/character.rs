use bevy::sprite::Anchor;
use crate::gameplay::character::stats::{StatsBundle, Strength};
use crate::gameplay::character::view::*;
use crate::gameplay::health::components::Health;
use crate::gameplay::health::view::HealthBarOffset;
use crate::prelude::*;
use crate::prelude::spawn::rounded_square::SpawnRoundedRectCommand;

pub mod stats;
mod view;

#[derive(Component)]
pub struct Character;

pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<TextureSlicer>()

            .add_systems(OnEnter(AppState::in_gameplay()), (
                spawn_character_root,
                spawn_character,
            ).chain())

            .add_systems(OnExit(AppState::in_gameplay()), despawn_character_root)

        ;
    }
}

pub fn spawn_character(
    mut commands: Commands,
    assets: Res<CharacterAssets>,
    root: Res<CharacterRoot>,
) {
    let sprite_handle = assets.bouncer.clone();
    let character = commands.spawn_with_name("character_bouncer")
        .insert(Character)
        .insert(StateScoped(AppState::in_gameplay()))
        .insert(UnitBundle {
            stats: StatsBundle {
                health: Health(balance::BOUNCER_HEALTH),
                strength: Strength(balance::BOUNCER_STRENGTH),
            },
            health_bar_offset: HealthBarOffset(constants::view::CHARACTER_HEALTH_BAR_OFFSET),
        })
        .insert(SpriteBundle {
            texture: sprite_handle,
            sprite: Sprite {
                anchor: Anchor::BottomCenter,
                ..default()
            },
            ..default()
        })
        .set_parent(root.0)
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