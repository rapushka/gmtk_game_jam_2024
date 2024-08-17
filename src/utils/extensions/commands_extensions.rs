use bevy::ecs::system::EntityCommands;
use crate::prelude::*;

pub trait CommandsExt {
    fn spawn_with_name(&mut self, name: &str) -> EntityCommands;
}

impl CommandsExt for Commands<'_, '_> {
    fn spawn_with_name(&mut self, name: &str) -> EntityCommands {
        self.spawn(Name::from(name))
    }
}