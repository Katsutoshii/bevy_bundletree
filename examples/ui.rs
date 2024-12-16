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
use bevy_bundletree::{ChildBundle, ChildBundles};
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

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let icon = asset_server.load("branding/icon.png");
    let light_logo = asset_server.load("branding/bevy_logo_light.png");
    let root = (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        PickingBehavior::IGNORE,
        // Left vertical fill
        ChildBundle((
            Node {
                width: Val::Px(200.),
                border: UiRect::all(Val::Px(2.)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.65, 0.65, 0.65)),
            // Flex column
            ChildBundle((
                Node {
                    width: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(5.)),
                    row_gap: Val::Px(5.),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                ChildBundles([
                    (
                        Text::new("Text Example"),
                        TextFont {
                            font: font.clone(),
                            font_size: 25.0,
                            ..default()
                        },
                        // Because this is a distinct label widget and
                        // not button/list item text, this is necessary
                        // for accessibility to treat the text accordingly.
                        Label,
                    ),
                    if cfg!(feature = "bevy_dev_tools") {
                        (
                            Text::new("Press Space to enable debug outlines."),
                            TextFont {
                                font: font.clone(),
                                ..default()
                            },
                            Label,
                        )
                    } else {
                        (
                            Text::new("Try enabling feature \"bevy_dev_tools\"."),
                            TextFont {
                                font: font.clone(),
                                ..default()
                            },
                            Label,
                        )
                    },
                ]),
            )),
        )),
        // Right vertical fill
        ChildBundle((
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Px(200.),
                ..default()
            },
            ChildBundle((
                Text::new("Scrolling list"),
                TextFont {
                    font: font.clone(),
                    font_size: 21.,
                    ..default()
                },
                Label,
            )),
            ChildBundle((
                Node {
                    flex_direction: FlexDirection::Column,
                    align_self: AlignSelf::Stretch,
                    height: Val::Percent(50.),
                    overflow: Overflow::scroll_y(),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.10, 0.10, 0.10)),
                ChildBundles((0..25).map({
                    let font = font.clone();
                    move |i| {
                        (
                            Text(format!("Item {i}")),
                            TextFont {
                                font: font.clone(),
                                ..default()
                            },
                            Label,
                            AccessibilityNode(Accessible::new(Role::ListItem)),
                            PickingBehavior {
                                should_block_lower: false,
                                ..default()
                            },
                        )
                    }
                })),
            )),
        )),
        ChildBundle((
            Node {
                left: Val::Px(210.),
                bottom: Val::Px(10.),
                position_type: PositionType::Absolute,
                ..default()
            },
            ChildBundle((
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
                ChildBundle((
                    ImageNode::new(light_logo),
                    // Uses the transform to rotate the logo image by 45 degrees
                    Transform::from_rotation(Quat::from_rotation_z(0.25 * PI)),
                    BorderRadius::all(Val::Px(10.)),
                    Outline {
                        width: Val::Px(2.),
                        offset: Val::Px(4.),
                        color: DARK_GRAY.into(),
                    },
                )),
            )),
        )),
        // Render order test
        ChildBundle((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            PickingBehavior::IGNORE,
            ChildBundle((
                Node {
                    width: Val::Px(100.0),
                    height: Val::Px(100.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(1.0, 0.0, 0.)),
                shadow,
                ChildBundles([
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
                    ),
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
                    ),
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
                    ),
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
                    ),
                ]),
            )),
        )),
        // Bevy logo
        ChildBundle((
            Node {
                width: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            ChildBundle((
                ImageNode::new(asset_server.load("branding/bevy_logo_dark_big.png"))
                    .with_mode(NodeImageMode::Stretch),
                Node {
                    width: Val::Px(500.0),
                    height: Val::Px(125.0),
                    margin: UiRect::top(Val::VMin(5.)),
                    ..default()
                },
                ChildBundle((
                    Node {
                        display: Display::None,
                        ..default()
                    },
                    Text::new("Bevy logo"),
                )),
            )),
        )),
        // Image flipping
        ChildBundle((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                column_gap: Val::Px(10.),
                padding: UiRect::all(Val::Px(10.)),
                ..default()
            },
            ChildBundles(
                [(false, false), (false, true), (true, true), (true, false)]
                    .iter()
                    .map(move |&(flip_x, flip_y)| {
                        (
                            ImageNode {
                                image: icon.clone(),
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
                    }),
            ),
        )),
    );
    commands.spawn(root);
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
