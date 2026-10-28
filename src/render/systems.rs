use bevy::prelude::*;

use crate::{render::Colored, spatial::TILE_SIZE};

pub(super) fn sync_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut square: Local<Option<Handle<Mesh>>>,
    colored_q: Query<(Entity, &Colored), Or<(Without<Mesh2d>, Without<MeshMaterial2d<ColorMaterial>>, Changed<Colored>)>>,
) {
    for (entity, colored) in &colored_q {
        let mesh = square.get_or_insert_with(|| meshes.add(Rectangle::new(TILE_SIZE, TILE_SIZE)));

        commands
            .entity(entity)
            .insert((Mesh2d(mesh.clone()), MeshMaterial2d(materials.add(colored.0))));
    }
}
