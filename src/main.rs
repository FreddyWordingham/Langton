use bevy::prelude::*;
use langton::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, LangtonPlugin))
        .add_systems(Update, trigger_spawn_item)
        .run();
}

fn trigger_spawn_item(
    mut spawn_item_msg: MessageWriter<SpawnThing<Item>>,
    mouse: Res<ButtonInput<MouseButton>>,
    cursor: Res<Cursor>,
) {
    let Some(pos) = cursor.0 else {
        return;
    };

    if mouse.pressed(MouseButton::Left) {
        spawn_item_msg.write(SpawnThing {
            thing: Item::IronOre,
            location: pos,
        });
    }
}
