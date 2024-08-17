use bevy::ecs::system::EntityCommands;
use crate::prelude::*;
use crate::prelude::spawn::rounded_square::SpawnRoundedRectCommand;
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
                justify_items:JustifyItems::Center,

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

// TODO: REMOVE ME
fn old_spawn_deck_panel(
    mut commands: Commands,
) {
    commands
        .spawn_with_name("gameplay HUD")
        .insert(NodeBundle {
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_with_name("background");
        });

    // ---
    let panel = commands
        .spawn_with_name("deck panel")
        .insert(StateScoped(AppState::in_gameplay()))
        .insert(InheritedVisibility::default())
        .insert(GlobalTransform::default())
        .insert(Transform::from_xyz(455.0, 0.0, 100.0))
        .id();

    commands.add(SpawnRoundedRectCommand {
        name: "Background",
        parent: Some(panel),
        color: Srgba::hex("5a4e44").unwrap().into(),
        size: Vec2::new(350.0, 700.0),
        ..default()
    });
}