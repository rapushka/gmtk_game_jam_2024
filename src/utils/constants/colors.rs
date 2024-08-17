use std::convert::Into;
use bevy::prelude::*;

pub fn default_button() -> Color { from_hex("c92941") }
pub fn hovered_button() -> Color { from_hex("ee4540") }
pub fn pressed_button() -> Color { from_hex("510a32") }

pub fn default_text() -> Color { from_hex("fbb86c") }
pub fn light_text() -> Color { from_hex("d2d2d2") }

fn from_hex(value: &'static str) -> Color { Srgba::hex(value).unwrap().into() }