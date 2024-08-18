use crate::gameplay::cards::types::CardType;
use crate::gameplay::character::stats::Strength;
use crate::gameplay::health::TakeDamage;
use crate::prelude::*;

pub struct InvokeCardPlugin;

impl Plugin for InvokeCardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<InvokeCard>()

            .observe(cast_attack_card)
        ;
    }
}

#[derive(Event)]
pub struct InvokeCard {
    pub card: CardType,
    pub sender: Entity,
    pub target: Entity,
}

#[allow(dead_code)]
fn ensure_all_cards_implemented(
    trigger: Trigger<InvokeCard>,
) {
    match trigger.event().card {
        CardType::Attack(_) => {}
    }
}

fn cast_attack_card(
    trigger: Trigger<InvokeCard>,
    mut commands: Commands,
    strength: Query<&Strength>,
) {
    let InvokeCard { card, sender, target, } = trigger.event();

    if let CardType::Attack(coefficient) = card {
        let strength = strength.get(*sender).expect("Can't use attack, if sender hasn't strength!").0;
        let damage = (*coefficient * strength as f32).round() as u8;

        commands.trigger_targets(TakeDamage(damage), *target);
    }
}