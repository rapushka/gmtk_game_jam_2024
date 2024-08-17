use bevy::text::Text;
use crate::prelude::{Color, default, Font, Handle, JustifyText, TextSection, TextStyle};

pub trait TextExt {
    fn to_text(self, font: Handle<Font>, font_size: f32, color: Color) -> Text;
}

impl TextExt for String {
    fn to_text(self,
               font: Handle<Font>,
               font_size: f32,
               color: Color,
    ) -> Text {
        Text {
            sections: vec![
                TextSection::new(
                    self,
                    TextStyle {
                        font,
                        font_size,
                        color,
                    },
                )],
            justify: JustifyText::Center,
            ..default()
        }
    }
}