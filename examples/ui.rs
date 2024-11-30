//! This example illustrates the various features of Bevy UI.

use accesskit::{Node as Accessible, Role};
use bevy::{
    a11y::AccessibilityNode,
    color::palettes::{basic::LIME, css::DARK_GRAY},
    input::mouse::{MouseScrollUnit, MouseWheel},
    picking::focus::HoverMap,
    prelude::*,
    ui::widget::NodeImageMode,
    winit::WinitSettings,
};
use bevy_bundletree::*;
use std::f32::consts::PI;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .add_systems(Update, update_scroll_position);

    #[cfg(feature = "bevy_dev_tools")]
    {
        app.add_plugins(bevy::dev_tools::ui_debug_overlay::DebugUiPlugin)
            .add_systems(Update, toggle_overlay);
    }

    app.run();
}

/// Define an enum to represent all possible node types in the UI tree.
// #[allow(clippy::large_enum_variant)]
#[derive(IntoBundleTree, BundleEnum)]
enum UiNode {
    Node(Node),
    PickingNode((Node, PickingBehavior)),
    BackgroundNode((Node, BackgroundColor)),
    BorderNode((Node, BorderColor, BackgroundColor)),
    BoxShadowNode((Node, BackgroundColor, BoxShadow)),
    Text((Text, TextFont, Label)),
    AccessibileText((Text, TextFont, Label, AccessibilityNode, PickingBehavior)),
    ImageNode((ImageNode, Transform, BorderRadius, Outline)),
    CustomImageNode((ImageNode, Node)),
    TextNode((Node, Text)),
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn((Camera2d, IsDefaultUiCamera, UiBoxShadowSamples(6)));

    let shadow = BoxShadow {
        color: Color::BLACK.with_alpha(0.5),
        blur_radius: Val::Px(2.),
        x_offset: Val::Px(10.),
        y_offset: Val::Px(10.),
        ..default()
    };

    let tree: BundleTree<UiNode> = (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        PickingBehavior::IGNORE,
    )
        .with_children([
            // left vertical fill (border)
            (
                Node {
                    width: Val::Px(200.),
                    border: UiRect::all(Val::Px(2.)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.65, 0.65, 0.65)),
            )
                .with_children([(
                    Node {
                        width: Val::Percent(100.),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(5.)),
                        row_gap: Val::Px(5.),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                )
                    .with_children([(
                        Text::new("Text Example"),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 25.0,
                            ..default()
                        },
                        // Because this is a distinct label widget and
                        // not button/list item text, this is necessary
                        // for accessibility to treat the text accordingly.
                        Label,
                    )
                        .into_tree()])
                    .with_children(if cfg!(feature = "bevy_dev_tools") {
                        [(
                            Text::new("Press Space to enable debug outlines."),
                            TextFont {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                ..default()
                            },
                            Label,
                        )
                            .into_tree()]
                    } else {
                        [(
                            Text::new("Try enabling feature \"bevy_dev_tools\"."),
                            TextFont {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                ..default()
                            },
                            Label,
                        )
                            .into_tree()]
                    })]),
            // right vertical fill
            (Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Px(200.),
                ..default()
            }
            .with_children([
                // Title
                (
                    Text::new("Scrolling list"),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 21.,
                        ..default()
                    },
                    Label,
                )
                    .into_tree(),
                // Scrolling list
                (
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_self: AlignSelf::Stretch,
                        height: Val::Percent(50.),
                        overflow: Overflow::scroll_y(),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.10, 0.10, 0.10)),
                )
                    .with_children(
                        (0..25)
                            .map(|i| {
                                (
                                    Text(format!("Item {i}")),
                                    TextFont {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        ..default()
                                    },
                                    Label,
                                    AccessibilityNode(Accessible::new(Role::ListItem)),
                                    PickingBehavior {
                                        should_block_lower: false,
                                        ..default()
                                    },
                                )
                                    .into_tree()
                            })
                            .collect::<Vec<_>>(),
                    ),
            ])),
            Node {
                left: Val::Px(210.),
                bottom: Val::Px(10.),
                position_type: PositionType::Absolute,
                ..default()
            }
            .with_children([(
                Node {
                    width: Val::Px(200.0),
                    height: Val::Px(200.0),
                    border: UiRect::all(Val::Px(20.)),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                BorderColor(LIME.into()),
                BackgroundColor(Color::srgb(0.8, 0.8, 1.)),
            )
                .with_children([(
                    ImageNode::new(asset_server.load("branding/bevy_logo_light.png")),
                    // Uses the transform to rotate the logo image by 45 degrees
                    Transform::from_rotation(Quat::from_rotation_z(0.25 * PI)),
                    BorderRadius::all(Val::Px(10.)),
                    Outline {
                        width: Val::Px(2.),
                        offset: Val::Px(4.),
                        color: DARK_GRAY.into(),
                    },
                )
                    .into_tree()])]),
            // render order test: reddest in the back, whitest in the front (flex center)
            (
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                PickingBehavior::IGNORE,
            )
                .with_children([(
                    Node {
                        width: Val::Px(100.0),
                        height: Val::Px(100.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(1.0, 0.0, 0.)),
                    shadow,
                )
                    .with_children([
                        (
                            Node {
                                // Take the size of the parent node.
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                position_type: PositionType::Absolute,
                                left: Val::Px(20.),
                                bottom: Val::Px(20.),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(1.0, 0.3, 0.3)),
                            shadow,
                        )
                            .into_tree(),
                        (
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                position_type: PositionType::Absolute,
                                left: Val::Px(40.),
                                bottom: Val::Px(40.),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(1.0, 0.5, 0.5)),
                            shadow,
                        )
                            .into_tree(),
                        (
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                position_type: PositionType::Absolute,
                                left: Val::Px(60.),
                                bottom: Val::Px(60.),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.0, 0.7, 0.7)),
                            shadow,
                        )
                            .into_tree(),
                        // alpha test
                        (
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                position_type: PositionType::Absolute,
                                left: Val::Px(80.),
                                bottom: Val::Px(80.),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(1.0, 0.9, 0.9, 0.4)),
                            BoxShadow {
                                color: Color::BLACK.with_alpha(0.3),
                                ..shadow
                            },
                        )
                            .into_tree(),
                    ])]),
            // bevy logo (flex center)
            Node {
                width: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexStart,
                ..default()
            }
            .with_children([(
                ImageNode::new(asset_server.load("branding/bevy_logo_dark_big.png"))
                    .with_mode(NodeImageMode::Stretch),
                Node {
                    width: Val::Px(500.0),
                    height: Val::Px(125.0),
                    margin: UiRect::top(Val::VMin(5.)),
                    ..default()
                },
            )
                .with_children([(
                    Node {
                        display: Display::None,
                        ..default()
                    },
                    Text::new("Bevy logo"),
                )
                    .into_tree()])]),
            // four bevy icons demonstrating image flipping
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                column_gap: Val::Px(10.),
                padding: UiRect::all(Val::Px(10.)),
                ..default()
            }
            .with_children(
                [(false, false), (false, true), (true, true), (true, false)]
                    .iter()
                    .map(|&(flip_x, flip_y)| {
                        (
                            ImageNode {
                                image: asset_server.load("branding/icon.png"),
                                flip_x,
                                flip_y,
                                ..default()
                            },
                            Node {
                                // The height will be chosen automatically to preserve the image's aspect ratio
                                width: Val::Px(75.),
                                ..default()
                            },
                        )
                            .into_tree()
                    })
                    .collect::<Vec<_>>(),
            ),
        ]);

    commands.spawn_tree(tree);
}

#[cfg(feature = "bevy_dev_tools")]
// The system that will enable/disable the debug outlines around the nodes
fn toggle_overlay(
    input: Res<ButtonInput<KeyCode>>,
    mut options: ResMut<bevy::dev_tools::ui_debug_overlay::UiDebugOptions>,
) {
    info_once!("The debug outlines are enabled, press Space to turn them on/off");
    if input.just_pressed(KeyCode::Space) {
        // The toggle method will enable the debug_overlay if disabled and disable if enabled
        options.toggle();
    }
}

/// Updates the scroll position of scrollable nodes in response to mouse input
pub fn update_scroll_position(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    mut scrolled_node_query: Query<&mut ScrollPosition>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        let (mut dx, mut dy) = match mouse_wheel_event.unit {
            MouseScrollUnit::Line => (mouse_wheel_event.x * 20., mouse_wheel_event.y * 20.),
            MouseScrollUnit::Pixel => (mouse_wheel_event.x, mouse_wheel_event.y),
        };

        if keyboard_input.pressed(KeyCode::ShiftLeft) || keyboard_input.pressed(KeyCode::ShiftRight)
        {
            std::mem::swap(&mut dx, &mut dy);
        }

        for (_pointer, pointer_map) in hover_map.iter() {
            for (entity, _hit) in pointer_map.iter() {
                if let Ok(mut scroll_position) = scrolled_node_query.get_mut(*entity) {
                    scroll_position.offset_x -= dx;
                    scroll_position.offset_y -= dy;
                }
            }
        }
    }
}
