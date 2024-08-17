use crate::prelude::{AssetServer, BuildChildren, ChildBuilder, Color, colors, CommandsExt, default, JustifyText, NodeBundle, Res, styles, Text, TextBundle, TextSection, TextStyle};

pub fn title(
    asset_server: &Res<AssetServer>,
    parent: &mut ChildBuilder,
    title_text: String,
) {
    parent.spawn(NodeBundle { style: styles::TITLE, ..default() })
        .with_children(|parent| {
            light_text(asset_server, title_text, parent, 64.0);
        });
}

pub fn text(
    asset_server: &Res<AssetServer>,
    text: String,
    parent: &mut ChildBuilder,
    font_size: f32,
) {
    parent
        .spawn_with_name(&format!("text: {text}"))
        .insert(text_bundle(asset_server, text, font_size));
}

pub fn light_text(
    asset_server: &Res<AssetServer>,
    text: String,
    parent: &mut ChildBuilder,
    font_size: f32,
) {
    parent
        .spawn_with_name(&format!("text: {text}"))
        .insert(light_text_bundle(asset_server, text, font_size));
}

pub fn text_bundle(
    asset_server: &Res<AssetServer>,
    text: String,
    font_size: f32,
) -> TextBundle {
    colored_text_bundle(asset_server, text, font_size, colors::default_text())
}

pub fn light_text_bundle(
    asset_server: &Res<AssetServer>,
    text: String,
    font_size: f32,
) -> TextBundle {
    colored_text_bundle(asset_server, text, font_size, colors::light_text())
}

fn colored_text_bundle(
    asset_server: &Res<AssetServer>,
    text: String,
    font_size: f32,
    color: Color,
) -> TextBundle {
    TextBundle {
        text: Text {
            sections: vec![
                TextSection::new(
                    text,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size,
                        color,
                    },
                )],
            justify: JustifyText::Center,
            ..default()
        },
        ..default()
    }
}