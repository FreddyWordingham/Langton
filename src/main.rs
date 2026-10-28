use bevy::prelude::*;
use langton::prelude::*;
use rand::RngExt;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, LangtonPlugin))
        .add_systems(Startup, trigger_spawn_terrain)
        .add_systems(Update, (trigger_spawn_machine, trigger_spawn_item))
        .run();
}

fn trigger_spawn_terrain(mut spawn_terrain_msg: MessageWriter<SpawnTerrain>, mut rng: ResMut<Rng>) {
    for _ in 0..10 {
        let x = rng.0.random_range(-20..=20);
        let y = rng.0.random_range(-20..=20);
        spawn_terrain_msg.write(SpawnTerrain {
            terrain: Terrain::Ore(Item::IronOre),
            location: ivec2(x, y),
        });
    }

    for _ in 0..10 {
        let x = rng.0.random_range(-20..=20);
        let y = rng.0.random_range(-20..=20);
        spawn_terrain_msg.write(SpawnTerrain {
            terrain: Terrain::Well(Item::Methane),
            location: ivec2(x, y),
        });
    }
}

fn trigger_spawn_machine(
    mut spawn_machine_msg: MessageWriter<SpawnMachine>,
    mouse: Res<ButtonInput<MouseButton>>,
    cursor: Res<Cursor>,
) {
    let Some(pos) = cursor.0 else {
        return;
    };

    if mouse.pressed(MouseButton::Right) {
        spawn_machine_msg.write(SpawnMachine {
            machine: Machine::Pumpjack,
            location: pos,
        });
    }
}

fn trigger_spawn_item(mut spawn_item_msg: MessageWriter<SpawnItem>, mouse: Res<ButtonInput<MouseButton>>, cursor: Res<Cursor>) {
    let Some(pos) = cursor.0 else {
        return;
    };

    if mouse.pressed(MouseButton::Left) {
        spawn_item_msg.write(SpawnItem {
            item: Item::IronOre,
            location: pos,
        });
    }
}
