use crate::gameplay::cards::{Card, DeckRoot};
use crate::prelude::*;
use crate::view::ui::create;

#[derive(Event)]
pub struct SpawnCard {
    pub name: String,
}

pub fn spawn_card(
    trigger: Trigger<SpawnCard>,
    mut commands: Commands,
    assets: Res<UiAssets>,
    deck_root: Query<Entity, With<DeckRoot>>,
) {
    info!("--- event received");
    
    for root in deck_root.iter() {
        let name = trigger.event().name.clone();

        let card_entity = commands
            .spawn_with_name(&format!("card: {}", name))
            .insert(Card)
            .insert(NodeBundle {
                background_color: colors::card_background_color().into(),
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