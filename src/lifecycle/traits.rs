use bevy::prelude::*;

pub(crate) trait ApplyTo: Component + Copy {
    fn apply_to(self, entity: &mut EntityCommands<'_>);
}
