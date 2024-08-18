use crate::prelude::*;

#[derive(Bundle)]
pub struct StatsBundle {
    pub health: Health,
    pub strength: Strength,
}

#[derive(Component)]
pub struct Strength(pub u8);