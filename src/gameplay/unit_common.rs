use crate::gameplay::character::stats::StatsBundle;
use crate::gameplay::health::view::HealthBarOffset;
use crate::prelude::*;

#[derive(Bundle)]
pub struct UnitBundle {
    pub stats: StatsBundle,
    pub health_bar_offset: HealthBarOffset,
}

#[derive(Component)]
pub struct Opponent(pub Entity);