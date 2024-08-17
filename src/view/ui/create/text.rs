use crate::prelude::*;

pub fn title(
    font: Handle<Font>,
    parent: &mut ChildBuilder,
    title_text: String,
) {
    parent.spawn(NodeBundle { style: styles::TITLE, ..default() })
        .with_children(|parent| {
            light_text(font, title_text, parent, 64.0);
        });
}

pub fn text(
    font: Handle<Font>,
    text: String,
    parent: &mut ChildBuilder,
    font_size: f32,
) {
    parent
        .spawn_with_name(&format!("text: {text}"))
        .insert(text_bundle(font, text, font_size));
}

pub fn light_text(
    font: Handle<Font>,
    text: String,
    parent: &mut ChildBuilder,
    font_size: f32,
) {
    parent
        .spawn_with_name(&format!("text: {text}"))
        .insert(light_text_bundle(font, text, font_size));
}

pub fn text_bundle(
    font: Handle<Font>,
    text: String,
    font_size: f32,
) -> TextBundle {
    colored_text_bundle(font, text, font_size, colors::default_text())
}

pub fn light_text_bundle(
    font: Handle<Font>,
    text: String,
    font_size: f32,
) -> TextBundle {
    colored_text_bundle(font, text, font_size, colors::light_text())
}

fn colored_text_bundle(
    font: Handle<Font>,
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
                        font,
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