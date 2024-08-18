use crate::gameplay::cards::{Card, DeckRoot};
use crate::gameplay::cards::types::CardType;
use crate::prelude::*;
use crate::view::ui::create;

#[derive(Event)]
pub struct SpawnCard {
    pub card_type: CardType,
}

pub fn spawn_card(
    trigger: Trigger<SpawnCard>,
    mut commands: Commands,
    assets: Res<UiAssets>,
    deck_root: Query<Entity, With<DeckRoot>>,
) {
    for root in deck_root.iter() {
        let event = trigger.event();
        let name = event.card_type.name();

        let card_entity = commands
            .spawn_with_name(&format!("card: {}", name))
            .insert(Card(event.card_type))
            .insert(NodeBundle {
                background_color: colors::card_background_color().into(),
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_content: AlignContent::Center,
                    align_items: AlignItems::Center,

                    padding: UiRect::all(Val::Px(5.0)),
                    margin: UiRect::all(Val::Px(5.0)),

                    ..default()
                },
                ..default()
            })
            .with_children(|item| {
                create::light_text(assets.font.clone(), name, item, view::CARD_NAME_FONT_SIZE)
            })
            .id();

        let mut parent = commands.entity(root);
        parent.add_child(card_entity);
    }
}

// fn apply(self, world: &mut World) {
//     let state = world.query::<Entity, With<DeckRoot>>();
//     let x = state.single();
// 
//     // .with_children(|parent| {
//     //     // List items
//     //     for i in 0..30 {
//     //         parent.spawn((
//     //             TextBundle::from_section(
//     //                 format!("Item {i}"),
//     //                 TextStyle {
//     //                     font,
//     //                     ..default()
//     //                 },
//     //             ),
//     //             Label,
//     //         ));
//     //     }
//     // })
// }