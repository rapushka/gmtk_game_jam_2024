use bevy::ecs::world::Command;
use crate::prelude::*;

pub struct SpawnRoundedRectCommand {
    pub name: &'static str,
    pub size: Vec2,
    pub position: Vec2,
    pub color: Color,
    pub parent: Option<Entity>,
}

impl Default for SpawnRoundedRectCommand {
    fn default() -> Self {
        Self {
            name: "rounded sprite",
            size: Vec2::new(64.0, 64.0),
            position: Vec2::ZERO,
            color: Srgba::WHITE.into(),
            parent: None,
        }
    }
}

impl Command for SpawnRoundedRectCommand {
    fn apply(self, world: &mut World) {
        let assets = world.resource::<CommonAssets>();
        let sprite_handle = assets.rounded_square.clone();

        let entity = world.spawn(Name::from(self.name))
            .insert(SpriteBundle {
                texture: sprite_handle,
                transform: Transform::from_translation(self.position.extend(0.0)),
                sprite: Sprite {
                    custom_size: Some(self.size),
                    color: self.color,
                    ..default()
                },
                ..default()
            })
            .insert(ImageScaleMode::Sliced(TextureSlicer {
                border: BorderRect::square(15.0),
                center_scale_mode: SliceScaleMode::Tile { stretch_value: 0.1 },
                sides_scale_mode: SliceScaleMode::Tile { stretch_value: 0.2 },
                max_corner_scale: 1.0,
            }))
            .id()
            ;

        if let Some(parent) = self.parent {
            let mut parent = world.entity_mut(parent);
            parent.add_child(entity);
        }
    }
}