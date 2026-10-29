use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Copy, Default, PartialEq)]
pub struct CameraControl {
    pub pan: Vec2,
    pub zoom: f32,
}
