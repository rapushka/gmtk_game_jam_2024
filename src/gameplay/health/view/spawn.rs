use crate::gameplay::health::view::{HealthBar, HealthBarOffset};
use crate::prelude::*;

pub fn add_health_view(
    trigger: Trigger<OnAdd, Health>,
    mut commands: Commands,
    health_bar_offset: Query<(&Health, &HealthBarOffset)>,
    assets: Res<UiAssets>,
) {
    let target = trigger.entity();
    let (health, offset) = health_bar_offset.get(target)
        .expect("entity must have HealthBarOffset if it has Health");

    let font = assets.font.clone();

    let health_bar = commands.spawn_with_name("health bar")
        .set_parent(target)
        .insert(Text2dBundle {
            text: health.0.to_string().to_text(font, view::HEALTH_FONT_SIZE, colors::health_color()),
            transform: Transform::from_translation(Vec3::Y * offset.0),
            ..default()
        })
        .id();

    commands.entity(target)
        .insert(HealthBar(health_bar))
    ;
}