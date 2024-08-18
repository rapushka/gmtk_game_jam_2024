use crate::prelude::*;

pub fn horizontal_layout(
    parent: &mut ChildBuilder,
    spawn_children: impl FnOnce(&mut ChildBuilder),
) {
    parent.spawn_with_name("horizontal layout")
        .insert(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,

                justify_content: JustifyContent::Center,
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::Stretch,
                align_items: AlignItems::Stretch,

                column_gap: Val::Px(10.0),
                margin: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            ..default()
        })
        .with_children(spawn_children);
}