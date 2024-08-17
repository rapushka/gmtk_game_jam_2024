use crate::gameplay::enemies::Enemy;
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

            .add_systems(Update, test_deal_damage)
        ;
    }
}

fn test_deal_damage(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    enemies: Query<Entity, (With<Health>, With<Enemy>)>,
) {
    if input.just_pressed(KeyCode::Space) {
        info!("--- Deal Damage is sent");

        for e in enemies.iter() {
            commands.trigger_targets(TakeDamage(1), e);
        }
    }
}