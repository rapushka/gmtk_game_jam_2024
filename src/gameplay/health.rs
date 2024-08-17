use crate::gameplay::health::death::Died;
use crate::gameplay::health::view::HealthViewPlugin;
use crate::prelude::*;
pub use self::delta::*;

pub mod view;
pub mod components;
mod delta;
mod death;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<Heal>()
            .add_event::<TakeDamage>()
            .add_event::<ChangeHealth>()
            .add_event::<HealthChanged>()
            .add_event::<Died>()

            .add_plugins(HealthViewPlugin)

            .register_type::<Health>()

            .observe(on_heal)
            .observe(on_damage_taken)
            .observe(change_health)
        ;
    }
}