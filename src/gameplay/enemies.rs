use crate::gameplay::character::stats::{StatsBundle, Strength};
use crate::gameplay::character::Character;
use crate::gameplay::enemies::ai::EnemyAiPlugin;
use crate::gameplay::enemies::cards::spawn_enemy_deck;
use crate::gameplay::enemies::enemy_type::EnemyType;
use crate::gameplay::health::components::Health;
use crate::gameplay::health::view::HealthBarOffset;
use crate::prelude::*;
use bevy::sprite::Anchor::BottomCenter;

pub mod enemy_type;
pub mod ai;
pub mod cards;

#[derive(Component)]
pub struct Enemy(pub EnemyType);

#[derive(Component)]
pub struct HasCards(pub Vec<Entity>);

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(EnemyAiPlugin)

            .observe(spawn_enemy_on_character_spawned)
            .observe(spawn_enemy_deck)
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

    let enemy_type = EnemyType::Rat;

    let enemy = commands.spawn_with_name(&enemy_type.name())
        .insert(StateScoped(AppState::in_gameplay()))
        .insert(Enemy(enemy_type))
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
        .insert(HasCards(Vec::new()))
        .id();

    commands.entity(character).insert(Opponent(enemy));

    // TODO: REMOVE?
    // # Cards Holder
    // let card_holder = commands.spawn_with_name("cards holder")
    //     .set_parent(enemy)
    //     .insert(NodeBundle {
    //         transform: Transform::from_xyz(0.0, view::ENEMY_CARDS_ROOT_OFFSET, 0.0),
    //         ..default()
    //     })
    //     .id();
    // 
    // commands.entity(enemy).insert(CardsHolder(card_holder));
}