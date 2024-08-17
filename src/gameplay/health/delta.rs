use bevy::a11y::accesskit::Role::Math;
use bevy::prelude::{Commands, Query, Trigger};
use crate::gameplay::health::death::Died;
use crate::prelude::{Event, Health};

#[derive(Event)]
pub struct Heal(pub u8);

#[derive(Event)]
pub struct TakeDamage(pub u8);

#[derive(Event)]
pub struct ChangeHealth(i16);

#[derive(Event)]
pub struct HealthChanged;

pub fn on_heal(
    trigger: Trigger<Heal>,
    mut commands: Commands,
) {
    let delta = trigger.event().0;
    commands.trigger_targets(ChangeHealth(delta as i16), trigger.entity());
}
pub fn on_damage_taken(
    trigger: Trigger<TakeDamage>,
    mut commands: Commands,
) {
    let delta = trigger.event().0;
    commands.trigger_targets(ChangeHealth(-(delta as i16)), trigger.entity());
}

pub fn change_health(
    trigger: Trigger<ChangeHealth>,
    mut commands: Commands,
    mut entities: Query<&mut Health>,
) {
    let target = trigger.entity();
    let mut health = entities.get_mut(target)
        .expect("can't change health if entity hasn't health");

    let delta = trigger.event().0;
    let new_health = health.0 as i16 + delta;

    let new_health = new_health.max(0);
    health.0 = new_health as u8;

    commands.trigger_targets(HealthChanged, target);

    if health.0 == 0 {
        commands.trigger_targets(Died, target);
    }
} 