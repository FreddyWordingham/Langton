use bevy::prelude::*;

pub trait ApplyTo: Component + Copy {
    fn apply_to(self, entity: &mut EntityCommands<'_>);
}
