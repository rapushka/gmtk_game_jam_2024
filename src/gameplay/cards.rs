use bevy::math::vec2;
use crate::prelude::*;
use crate::utils::spawn::rounded_square::SpawnRoundedRectCommand;

pub struct CardsPlugin;

impl Plugin for CardsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::in_gameplay()), spawn_deck_panel)
        ;
    }
}

fn spawn_deck_panel(
    mut commands: Commands,
) {
    let panel = commands
        .spawn_with_name("deck panel")
        .insert(StateScoped(AppState::in_gameplay()))
        .insert(InheritedVisibility::default())
        .insert(GlobalTransform::default())
        .insert(Transform::from_xyz(455.0, 0.0, 100.0))
        .id();

    commands.add(SpawnRoundedRectCommand {
        name: "Background",
        parent: Some(panel),
        color: Srgba::hex("5a4e44").unwrap().into(),
        size: Vec2::new(350.0, 700.0),
        ..default()
    });
}