use crate::gameplay::cards::DeckRoot;
use crate::prelude::*;
use crate::view::ui::create;
use crate::view::ui::gameplay::AutoPlayButton;

pub fn spawn_gameplay_hud(
    mut commands: Commands,
    assets: Res<UiAssets>,
) {
    let sprite_handle = assets.rounded_square.clone();

    commands
        .spawn_with_name("gameplay HUD")
        .insert(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),

                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|root| {
            root
                .spawn_with_name("space")
                .insert(NodeBundle {
                    style: Style {
                        width: Val::Auto,
                        ..default()
                    },
                    ..default()
                })
            ;

            spawn_deck_panel(sprite_handle, root, &assets);
        })
    ;
}

fn spawn_deck_panel(
    sprite_handle: Handle<Image>,
    root: &mut ChildBuilder,
    assets: &Res<UiAssets>,
) {
    let font = assets.font.clone();

    root
        .spawn_with_name("deck panel")
        .insert(NodeBundle {
            style: Style {
                height: Val::Percent(100.0),
                width: Val::Px(view::DECK_PANEL_WIDTH),

                padding: UiRect::all(Val::Px(20.0)),

                ..default()
            },
            ..default()
        })
        .with_children(|right_dock| {
            panel(sprite_handle, right_dock, |panel| {
                create::list_view(panel, DeckRoot, Val::Percent(75.0));

                create::button(font, panel, "play".to_string(), AutoPlayButton);
            });
        })
    ;
}

fn panel(
    sprite_handle: Handle<Image>,
    parent: &mut ChildBuilder,
    build_children: impl FnOnce(&mut ChildBuilder),
) {
    parent
        .spawn_with_name("background")
        .insert(ImageBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::End,
                align_items: AlignItems::End,
                justify_items: JustifyItems::Center,

                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            image: UiImage::new(sprite_handle).with_color(colors::background()),
            ..default()
        })
        .insert(ImageScaleMode::Sliced(TextureSlicer {
            border: BorderRect::square(15.0),
            center_scale_mode: SliceScaleMode::Stretch,
            sides_scale_mode: SliceScaleMode::Stretch,
            max_corner_scale: 1.0,
        }))
        .with_children(build_children)
    ;
}