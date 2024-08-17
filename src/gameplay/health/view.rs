use self::spawn::add_health_view;
use crate::prelude::*;

mod spawn;

#[derive(Component)]
pub struct HealthBarOffset(pub f32);

pub struct HealthViewPlugin;

impl Plugin for HealthViewPlugin {
    fn build(&self, app: &mut App) {
        app
            .observe(add_health_view)
        ;
    }
}