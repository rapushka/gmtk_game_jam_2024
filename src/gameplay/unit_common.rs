use crate::gameplay::health::view::HealthBarOffset;
use crate::prelude::*;

#[derive(Bundle)]
pub struct UnitBundle {
    pub health: Health,
    pub health_bar_offset: HealthBarOffset,
}