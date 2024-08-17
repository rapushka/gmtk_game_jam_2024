use crate::prelude::*;

#[derive(Component, Reflect)]
pub struct Health(pub u8);

#[derive(Event)]
struct Heal(pub u8);
#[derive(Event)]
struct TakeDamage(pub u8);
#[derive(Event)]
pub(super) struct ChangeHealth(u8);

