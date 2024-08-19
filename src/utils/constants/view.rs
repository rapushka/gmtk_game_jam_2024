use crate::prelude::Vec2;

// Common
pub const DEFAULT_MOVEMENT_SPEED: f32 = 750.0;

// Deck & Cards
pub const DECK_PANEL_WIDTH: f32 = 350.0;
pub const CARD_NAME_FONT_SIZE: f32 = 15.0;
pub const CARD_ITEM_SIZE: Vec2 = Vec2 { x: 200.0, y: 25.0 };
pub const CARD_ITEM_SPACING: f32 = 35.0;

// # Health Bars
pub const CHARACTER_HEALTH_BAR_OFFSET: f32 = -20.0;
pub const ENEMY_HEALTH_BAR_OFFSET: f32 = 185.0;
pub const ENEMY_CARDS_ROOT_OFFSET: f32 = 250.0;

pub const HEALTH_FONT_SIZE: f32 = 30.0;

pub mod timings {
    pub const ENEMY_THINKING_SECONDS: f32 = 0.25;
    pub const TURN_PASS_DELAY: f32 = 0.5;
}