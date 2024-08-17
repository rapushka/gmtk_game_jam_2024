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
                flex_direction: FlexDirection::ColumnReverse,
                align_self: AlignSelf::Stretch,
                align_items: AlignItems::Stretch,

                justify_items: JustifyItems::Start,
                align_content: AlignContent::End,
                justify_content: JustifyContent::End,

                height,
                overflow: Overflow::clip_y(),
                ..default()
            },
            background_color: colors::background().into(),
            ..default()
        })
        .insert(component)
    ;
}