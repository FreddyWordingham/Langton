use bevy::prelude::*;

use crate::render::Appearance;

pub(super) fn sync_appearances<A: Appearance>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    appearance_q: Query<(Entity, &A), Or<(Without<Sprite>, Changed<A>)>>,
) {
    for (ent, appearance) in &appearance_q {
        commands.entity(ent).insert(Sprite {
            image: asset_server.load(appearance.sprite()),
            color: appearance.color(),
            ..default()
        });
    }
}
