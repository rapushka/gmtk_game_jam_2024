use crate::prelude::*;

pub fn list_view<C: Component>(
    parent: &mut ChildBuilder,
    component: C,
    height: Val,
) {
    parent
        .spawn_with_name("list view")
        .insert(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Stretch,
                height,
                overflow: Overflow::clip_y(),
                ..default()
            },
            background_color: colors::background().into(),
            ..default()
        })
        .with_children(|list_view| {
            list_view
                .spawn_with_name("moving panel")
                .insert((
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    },
                ))
                // .with_children(|parent| {
                //     // List items
                //     for i in 0..30 {
                //         parent.spawn((
                //             TextBundle::from_section(
                //                 format!("Item {i}"),
                //                 TextStyle {
                //                     font,
                //                     ..default()
                //                 },
                //             ),
                //             Label,
                //         ));
                //     }
                // })
                .insert(component)
            ;
        });
}