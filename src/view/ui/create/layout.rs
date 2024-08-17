use crate::prelude::{BuildChildren, ChildBuilder, default, FlexDirection, NodeBundle, Style, UiRect, Val};

pub fn horizontal_layout(
    parent: &mut ChildBuilder,
    spawn_children: impl FnOnce(&mut ChildBuilder),
) {
    parent.spawn(
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(10.0),
                margin: UiRect::all(Val::Px(25.0)),
                ..default()
            },
            ..default()
        }
    )
        .with_children(spawn_children);
}