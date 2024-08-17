use crate::gameplay::health::HealthChanged;
use self::spawn::add_health_view;
use crate::prelude::*;

mod spawn;

#[derive(Component)]
pub struct HealthBarOffset(pub f32);

#[derive(Component)]
pub struct HealthBar(pub Entity);

pub struct HealthViewPlugin;

impl Plugin for HealthViewPlugin {
    fn build(&self, app: &mut App) {
        app
            .observe(add_health_view)
            .observe(update_health_bar_on_health_changed)
        ;
    }
}

fn update_health_bar_on_health_changed(
    trigger: Trigger<HealthChanged>,
    health_bars: Query<(&HealthBar, &Health)>,
    mut texts: Query<&mut Text>,
) {
    let target = trigger.entity();

    let (health_bar, health) = health_bars.get(target)
        .expect("entity must have health and health bar");

    let current_health = health.0;

    let mut text = texts.get_mut(health_bar.0).expect("can't get Text from the Health Bar");
    text.sections[0].value = current_health.to_string();
}