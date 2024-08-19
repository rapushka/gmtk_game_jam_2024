use crate::prelude::*;
use bevy::prelude::Entity;

#[derive(Resource)]
pub struct CharacterRoot(pub Entity);

pub fn spawn_character_root(
    mut commands: Commands,
) {
    let entity = commands
        .spawn_with_name("characters root")
        .insert(StateScoped(AppState::in_gameplay()))
        .insert(Transform::from_xyz(-250.0, -120.0, 0.0))
        .insert(GlobalTransform::default())
        .insert(InheritedVisibility::default())
        .id();

    commands.insert_resource(CharacterRoot(entity));
}

pub fn despawn_character_root(
    mut commands: Commands,
) {
    commands.remove_resource::<CharacterRoot>();
}