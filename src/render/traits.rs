use bevy::prelude::*;

pub(crate) trait Appearance: Component {
    fn sprite(&self) -> &'static str;

    fn color(&self) -> Color;
}
