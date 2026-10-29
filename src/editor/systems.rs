use bevy::{
    picking::hover::Hovered,
    prelude::*,
    ui_widgets::{Activate, Button},
};

use crate::{
    cursor::Cursor,
    editor::{
        ActiveEditorSelection, ActiveEditorTab, EditorMenu, EditorSelection, EditorSelectionButton, EditorSelectionPanel,
        EditorTab, EditorTabButton, EditorTabPanel, ShowEditorUi, TOGGLE_EDITOR_UI_KEY,
    },
    lifecycle::{SpawnItem, SpawnMachine, SpawnTerrain},
    render::Appearance,
    thing::{Item, Machine, Terrain},
};

pub(super) fn toggle_editor_ui(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    show_editor_ui: Option<Res<ShowEditorUi>>,
) {
    if keyboard.just_pressed(TOGGLE_EDITOR_UI_KEY) {
        if show_editor_ui.is_some() {
            commands.remove_resource::<ShowEditorUi>();
        } else {
            commands.insert_resource(ShowEditorUi);
        }
    }
}

pub(super) fn spawn_menu(mut commands: Commands, editor_menu_q: Query<(), With<EditorMenu>>) {
    if !editor_menu_q.is_empty() {
        return;
    }

    commands
        .spawn((
            EditorMenu,
            Node {
                width: px(200.0),
                height: percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: px(5.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    EditorTabPanel,
                    Node {
                        width: percent(100.0),
                        height: auto(),
                        padding: UiRect::all(px(5.0)),
                        column_gap: px(5.0),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    for tab in EditorTab::ALL {
                        parent
                            .spawn((
                                EditorTabButton(tab),
                                Button,
                                Hovered::default(),
                                Node {
                                    flex_grow: 1.0,
                                    flex_basis: px(0.0),
                                    min_width: px(0.0),
                                    height: px(30.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                children![Text::new(tab.as_str())],
                            ))
                            .observe(
                                |event: On<Activate>,
                                 mut commands: Commands,
                                 active_tab: Option<Res<ActiveEditorTab>>,
                                 button_q: Query<&EditorTabButton>| {
                                    let Ok(button) = button_q.get(event.entity) else {
                                        return;
                                    };
                                    let tab = button.0;

                                    if active_tab.as_ref().is_some_and(|active| active.0 == tab) {
                                        commands.remove_resource::<ActiveEditorTab>();
                                    } else {
                                        commands.insert_resource(ActiveEditorTab(tab));
                                        info!("Active editor tab set to {:?}", tab);
                                    }
                                },
                            );
                    }
                });
        });
}

pub(super) fn despawn_menu(mut commands: Commands, editor_menu_q: Query<Entity, With<EditorMenu>>) {
    for ent in editor_menu_q {
        commands.entity(ent).despawn();
    }
    commands.remove_resource::<ActiveEditorTab>();
    commands.remove_resource::<ActiveEditorSelection>();
}

pub(super) fn spawn_selection_panel(
    mut commands: Commands,
    assets: Res<AssetServer>,
    active_tab: Res<ActiveEditorTab>,
    editor_menu_q: Query<Entity, With<EditorMenu>>,
    editor_selection_panel_q: Query<Entity, With<EditorSelectionPanel>>,
) {
    for ent in editor_selection_panel_q {
        commands.entity(ent).despawn();
    }

    for ent in editor_menu_q {
        commands.entity(ent).with_children(|parent| {
            parent
                .spawn((
                    EditorSelectionPanel,
                    Node {
                        width: percent(100.0),
                        height: percent(100.0),
                        padding: UiRect::all(px(5.0)),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(5.0),
                        ..default()
                    },
                ))
                .with_children(|parent| match active_tab.0 {
                    EditorTab::Terrain => {
                        for terrain in Terrain::ALL {
                            parent
                                .spawn(selection_button(
                                    EditorSelection::Terrain(terrain),
                                    terrain.as_str(),
                                    ImageNode::new(assets.load(terrain.sprite())).with_color(terrain.item().color()),
                                ))
                                .observe(select_button);
                        }
                    }
                    EditorTab::Machine => {
                        for machine in Machine::ALL {
                            parent
                                .spawn(selection_button(
                                    EditorSelection::Machine(machine),
                                    machine.as_str(),
                                    ImageNode::new(assets.load(machine.sprite())).with_color(machine.color()),
                                ))
                                .observe(select_button);
                        }
                    }
                    EditorTab::Item => {
                        for item in Item::ALL {
                            parent
                                .spawn(selection_button(
                                    EditorSelection::Item(item),
                                    item.as_str(),
                                    ImageNode::new(assets.load(item.sprite())).with_color(item.color()),
                                ))
                                .observe(select_button);
                        }
                    }
                });
        });
    }
}

pub(super) fn despawn_selection_panel(
    mut commands: Commands,
    editor_selection_panel_q: Query<Entity, With<EditorSelectionPanel>>,
) {
    for ent in editor_selection_panel_q {
        commands.entity(ent).despawn();
    }
    commands.remove_resource::<ActiveEditorSelection>();
}

pub(super) fn trigger_spawn_thing(
    mut spawn_terrain_msg: MessageWriter<SpawnTerrain>,
    mut spawn_machine_msg: MessageWriter<SpawnMachine>,
    mut spawn_item_msg: MessageWriter<SpawnItem>,
    mouse: Res<ButtonInput<MouseButton>>,
    cursor: Res<Cursor>,
    active_editor_selection: Res<ActiveEditorSelection>,
) {
    let Some(pos) = cursor.0 else {
        return;
    };

    if mouse.pressed(MouseButton::Left) {
        match active_editor_selection.0 {
            EditorSelection::Terrain(terrain) => {
                spawn_terrain_msg.write(SpawnTerrain { terrain, location: pos });
            }
            EditorSelection::Machine(machine) => {
                spawn_machine_msg.write(SpawnMachine { machine, location: pos });
            }
            EditorSelection::Item(item) => {
                spawn_item_msg.write(SpawnItem { item, location: pos });
            }
        }
    }
}

fn selection_button(selection: EditorSelection, label: &str, image: ImageNode) -> impl Bundle {
    (
        EditorSelectionButton(selection),
        Button,
        Hovered::default(),
        Node {
            width: percent(100.0),
            height: px(30.0),
            padding: UiRect::all(px(5.0)),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        },
        children![
            (
                Text::new(label),
                TextFont {
                    font_size: FontSize::Px(16.0),
                    ..default()
                }
            ),
            (
                Node {
                    width: px(16.0),
                    height: px(16.0),
                    ..default()
                },
                image
            )
        ],
    )
}

fn select_button(event: On<Activate>, mut commands: Commands, buttons: Query<&EditorSelectionButton>) {
    let Ok(button) = buttons.get(event.entity) else {
        return;
    };
    commands.insert_resource(ActiveEditorSelection(button.0));
}
