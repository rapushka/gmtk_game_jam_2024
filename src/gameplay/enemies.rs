use bevy::sprite::Anchor::BottomCenter;
use bevy::utils::tracing::Instrument;
use crate::gameplay::character::Character;
use crate::gameplay::character::stats::{StatsBundle, Strength};
use crate::gameplay::health::components::Health;
use crate::gameplay::health::view::HealthBarOffset;
use crate::prelude::*;

#[derive(Component)]
pub struct Enemy;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .observe(spawn_enemy_on_character_spawned)
        ;
    }
}

fn spawn_enemy_on_character_spawned(
    trigger: Trigger<OnAdd, Character>,
    mut commands: Commands,
    assets: Res<EnemyAssets>,
) {
    let sprite_handle = assets.rat.clone();
    let character = trigger.entity();

    let enemy = commands.spawn_with_name("rat")
        .insert(StateScoped(AppState::in_gameplay()))
        .insert(Enemy)
        .insert(UnitBundle {
            stats: StatsBundle {
                health: Health(balance::RAT_HEALTH),
                strength: Strength(balance::RAT_STRENGTH),
            },
            health_bar_offset: HealthBarOffset(view::ENEMY_HEALTH_BAR_OFFSET),
        })
        .insert(Opponent(character))
        .set_parent(character)
        .insert(SpriteBundle {
            texture: sprite_handle,
            sprite: Sprite {
                anchor: BottomCenter,
                ..default()
            },
            ..default()
        })
        .insert(Transform::from_xyz(0.0, 100.0, -1.0))
        .id();

    commands.entity(character).insert(Opponent(enemy));
}