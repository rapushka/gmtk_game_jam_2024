use crate::prelude::*;
use crate::view::ui::create;

pub fn button<C>(
    font: Handle<Font>,
    parent: &mut ChildBuilder,
    string: String,
    component: C,
)
where
    C: Component,
{
    button_internal(font, parent, string, component, styles::BUTTON);
}

pub fn small_button<C>(
    font: Handle<Font>,
    parent: &mut ChildBuilder,
    string: String,
    component: C,
)
where
    C: Component,
{
    button_internal(font, parent, string, component, styles::SMALL_BUTTON);
}

fn button_internal<C>(
    font: Handle<Font>,
    parent: &mut ChildBuilder,
    string: String,
    component: C,
    style: Style,
) where
    C: Component,
{
    parent
        .spawn_with_name("button")
        .insert((
            component,
            ButtonBundle {
                style,
                background_color: colors::default_button().into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            create::light_text(font, string, parent, 32.0);
        });
}

pub fn image_button<C>(
    asset_server: &Res<AssetServer>,
    parent: &mut ChildBuilder,
    component: C,
    path: &'static str,
    width: f32,
) where
    C: Component,
{
    parent.spawn((
        component,
        ButtonBundle {
            style: styles::square_button(width),
            background_color: colors::default_button().into(),
            ..default()
        },
    ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    // This will set the logo to be 200px wide, and auto adjust its height
                    width: Val::Px(width),
                    ..default()
                },
                image: UiImage::new(asset_server.load(path)),
                ..default()
            });
        });
}