use bevy::ecs::world::Command;
use crate::prelude::*;

pub struct SpawnTextCommand {
    pub entity: Option<Entity>,
    pub text: String,
    pub font_size: f32,
    pub color: Color,
    pub position: Vec2,
    pub parent: Option<Entity>,
}

impl Default for SpawnTextCommand {
    fn default() -> Self {
        Self {
            entity: None,
            text: String::new(),
            font_size: 24.0,
            color: colors::default_text(),
            position: Vec2::ZERO,
            parent: None,
        }
    }
}

impl Command for SpawnTextCommand {
    fn apply(self, world: &mut World) {
        let font = world.resource::<UiAssets>().font.clone();
        let mut commands = world.commands();

        let mut entity_command = match self.entity {
            Some(e) => commands.entity(e),
            None => commands.spawn_with_name(&format!("text: {}", self.text)),
        };

        let entity = entity_command
            .insert(Text2dBundle {
                text: self.text.to_text(font, self.font_size, self.color),
                transform: Transform::from_translation(self.position.extend(1.0)),
                ..default()
            })
            .id();

        if let Some(parent) = self.parent {
            world.entity_mut(parent).add_child(entity);
        }
    }
}