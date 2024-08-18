use bevy::ecs::world::Command;
use crate::prelude::*;

pub struct SpawnRoundedRectCommand {
    /// if None - the command will spawn a new entity
    pub entity: Option<Entity>,
    pub name: &'static str,
    pub size: Vec2,
    pub position: Vec2,
    pub is_under_parent: bool,
    pub color: Color,
    pub parent: Option<Entity>,
}

impl Default for SpawnRoundedRectCommand {
    fn default() -> Self {
        Self {
            name: "rounded sprite",
            size: Vec2::new(64.0, 64.0),
            position: Vec2::ZERO,
            is_under_parent: true,
            color: Srgba::WHITE.into(),
            parent: None,
            entity: None,
        }
    }
}

impl Command for SpawnRoundedRectCommand {
    fn apply(self, world: &mut World) {
        let assets = world.resource::<UiAssets>();
        let sprite_handle = assets.rounded_square.clone();

        let z_position = if self.is_under_parent { -1.0 } else { 1.0 };

        let mut commands = world.commands();
        let mut entity_command = match self.entity {
            Some(e) => commands.entity(e),
            None => commands.spawn_with_name(self.name),
        };

        entity_command
            .insert(SpriteBundle {
                texture: sprite_handle,
                transform: Transform::from_translation(self.position.extend(z_position)),
                sprite: Sprite {
                    custom_size: Some(self.size),
                    color: self.color,
                    ..default()
                },
                ..default()
            })
            .insert(ImageScaleMode::Sliced(TextureSlicer {
                border: BorderRect::square(15.0),
                center_scale_mode: SliceScaleMode::Stretch,
                sides_scale_mode: SliceScaleMode::Stretch,
                max_corner_scale: 1.0,
            }));

        if let Some(parent) = self.parent {
            entity_command.set_parent(parent);
        }
    }
}