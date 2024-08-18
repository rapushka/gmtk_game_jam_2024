use crate::gameplay::cards::Card;
use crate::prelude::*;

#[derive(Resource, Default)]
pub struct CardCount(u8);

#[derive(Component, Reflect)]
pub struct CardOrder(u8);

pub struct CardOrderingPlugin;

impl Plugin for CardOrderingPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<CardOrder>()

            .init_resource::<CardCount>()

            .observe(add_card_order)
        ;
    }
}

fn add_card_order(
    trigger: Trigger<OnAdd, Card>,
    mut commands: Commands,
    mut card_count: ResMut<CardCount>,
) {
    let card_entity = trigger.entity();
    let current_count = card_count.0;

    commands.entity(card_entity)
        .insert(CardOrder(current_count))
    ;

    card_count.0 += 1;
}