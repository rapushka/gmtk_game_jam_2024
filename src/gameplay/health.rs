use crate::prelude::*;

pub mod view;
pub mod components;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Health>()
        ;
    }
}