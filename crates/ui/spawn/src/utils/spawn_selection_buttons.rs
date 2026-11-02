use arc_button::{button, button_node};
use arc_construct::Construct;
use arc_draw::Draw;
use arc_feature::Feature;
use arc_item::Item;
use arc_terrain::Terrain;
use bevy::{prelude::*, ui_widgets::Activate};

use crate::{resources::ActiveSpawnSelection, types::SpawnTab};

pub(crate) fn spawn_selection_buttons(parent: &mut ChildSpawnerCommands<'_>, assets: &AssetServer, spawn_tab: SpawnTab) {
    match spawn_tab {
        SpawnTab::Terrain => {
            for terrain in Terrain::ALL {
                spawn_button(parent, assets, &terrain).observe(move |_: On<Activate>, mut commands: Commands| {
                    commands.insert_resource(ActiveSpawnSelection::Terrain(terrain));
                });
            }
        }
        SpawnTab::Feature => {
            for feature in Feature::ALL {
                spawn_button(parent, assets, &feature).observe(move |_: On<Activate>, mut commands: Commands| {
                    commands.insert_resource(ActiveSpawnSelection::Feature(feature));
                });
            }
        }
        SpawnTab::Construct => {
            for construct in Construct::ALL {
                spawn_button(parent, assets, &construct).observe(move |_: On<Activate>, mut commands: Commands| {
                    commands.insert_resource(ActiveSpawnSelection::Construct(construct));
                });
            }
        }
        SpawnTab::Item => {
            for item in Item::ALL {
                spawn_button(parent, assets, &item).observe(move |_: On<Activate>, mut commands: Commands| {
                    commands.insert_resource(ActiveSpawnSelection::Item(item));
                });
            }
        }
    }
}

fn spawn_button<'a>(parent: &'a mut ChildSpawnerCommands<'_>, assets: &AssetServer, draw: &impl Draw) -> EntityCommands<'a> {
    parent.spawn((
        button(Node {
            width: percent(100.0),
            height: auto(),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..button_node()
        }),
        children![
            Text::new(draw.name()),
            (
                Node {
                    width: px(16.0),
                    height: px(16.0),
                    ..default()
                },
                ImageNode::new(assets.load(draw.sprite_path())).with_color(draw.color())
            )
        ],
    ))
}
