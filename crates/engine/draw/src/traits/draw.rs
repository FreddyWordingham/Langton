use bevy::prelude::*;

pub trait Draw {
    fn name(&self) -> &'static str;

    fn sprite_path(&self) -> &'static str;

    fn color(&self) -> Color;
}
