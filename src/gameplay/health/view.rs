use crate::gameplay::health::components::Health;
use crate::prelude::*;

#[derive(Component)]
pub struct HealthbarOffset(f32);

pub struct HealthViewPlugin;

impl Plugin for HealthViewPlugin {
    fn build(&self, app: &mut App) {
        app
            .observe(add_health_view)
        ;
    }
}

fn add_health_view(
    trigger: Trigger<OnAdd, Health>,
    mut commands: Commands,
) {
    
}
