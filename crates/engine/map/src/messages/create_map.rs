use bevy::prelude::*;

use crate::resources::Map;

#[derive(Message)]
pub struct CreateMap {
    pub map: Map,
}
