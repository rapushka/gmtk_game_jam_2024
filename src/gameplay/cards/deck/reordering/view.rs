use bevy::prelude::{Changed, Commands, Query};
use crate::gameplay::cards::deck::reordering::CardOrder;
use crate::prelude::*;
use crate::view::move_to_target::TargetPosition;

pub fn order_views(
    mut commands: Commands,
    cards: Query<(Entity, &Transform, &CardOrder), Changed<CardOrder>>,
) {
    for (entity, transform, card_order) in cards.iter() {
        let y = card_order.0 as f32 * view::CARD_ITEM_SPACING;

        let target_position = Vec3 {
            y,
            ..transform.translation
        };
        commands.entity(entity).insert(TargetPosition(target_position));
    }
}

#[cfg(test)]
mod tests {
    use crate::view::move_to_target::TargetPosition;
    use super::*;

    const CARD_HEIGHT: f32 = view::CARD_ITEM_SIZE.y;
    const SPACING: f32 = view::CARD_ITEM_SPACING;

    #[test]
    fn _010_when_card_order_is_0_then_target_y_should_be_0() {
        let card_order = 0;
        let expected_y = 0.0;

        test_target_y(expected_y, card_order);
    }

    #[test]
    fn _020_when_card_order_is_1_then_target_y_should_be_1_spacing() {
        let card_order = 1;
        let expected_y = SPACING;

        test_target_y(expected_y, card_order);
    }

    #[test]
    fn _030_when_card_order_is_4_then_target_y_should_be_4_spacing() {
        let card_order = 4;
        let expected_y = 4.0 * SPACING;

        test_target_y(expected_y, card_order);
    }

    fn test_target_y(expected_y: f32, card_order: u8) {
        // Arrange.
        let mut app = App::new();
        let entity = app.world_mut()
            .spawn_empty()
            .insert(CardOrder(card_order))
            .insert(Transform::default())
            .id();

        // Act.
        app.add_systems(Update, order_views);
        app.update();

        // Assert.
        let target_position = app.world().get::<TargetPosition>(entity).unwrap();
        let target_y = target_position.0.y;
        assert_eq!(target_y, expected_y);
    }
}