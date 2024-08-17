use crate::gameplay::health::view::HealthViewPlugin;
use crate::prelude::*;

pub mod view;
pub mod components;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(HealthViewPlugin)

            .register_type::<Health>()
        ;
    }
}