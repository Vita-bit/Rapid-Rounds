use bevy::prelude::*;
use crate::session::states::*;
use bevy::picking::hover::*;
use bevy::ui::Pressed;
use bevy::ui_widgets::{Button, Activate, observe};

use crate::homescreen::substates::choosing_game::sub_plugin::ChoosingGamePlugin;
use crate::homescreen::substates::menu::sub_plugin::MenuPlugin;


pub struct HomescreenPlugin;

#[derive(Component, Default)]
pub struct HomescreenCamera {
    velocity: Vec3,
}

#[derive(Component)]
struct RRStartgameButton;

#[derive(Component)]
struct RRSettingsButton;

#[derive(Component)]
struct RRCameraPosInfo;

#[derive(SubStates, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[source(AppState = AppState::Homescreen)]
pub enum HomescreenSubstates {
    #[default]
    Menu,
    ChoosingGame,
    Loading,
}

impl Plugin for HomescreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_sub_state::<HomescreenSubstates>()
            .add_plugins((ChoosingGamePlugin, MenuPlugin))
            .add_systems
                (
                    OnEnter(AppState::Homescreen),
                    (
                        setup_homescreen,
                        setup_homescreen_buttons,
                        test_3d
                    ).chain()
                )
            .add_systems
                (
                    Update,
                    (
                        homescreen_input,
                        update_startgame_button_appearance,
                        update_settings_button_appearance,
                        move_camera_input,
                        update_camera_info
                    )
                    .run_if(in_state(AppState::Homescreen))
                );
    }
}

fn setup_homescreen(mut commands: Commands) {
    commands.insert_resource(ClearColor(Color::hsl(0f32, 0f32, 1f32)));

    commands.spawn((
        Camera3d::default(),
        HomescreenCamera::default(),
        Transform::from_xyz(15f32, 5f32, 10f32).looking_at(Vec3::ZERO, Vec3::Y),
        DespawnOnExit(AppState::Homescreen),
    ));
}

fn homescreen_input(keys: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<AppState>>) {
    if keys.just_pressed(KeyCode::Digit1) {
        next.set(AppState::Lobby);
    }
}

fn move_camera_input
(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    camera: Single<(&mut Transform, &mut HomescreenCamera)>
)
{
    let (mut transform, mut cam) = camera.into_inner();

    let speed = 0.5f32;
    let smoothing = 6f32;
    let forward = transform.forward().with_y(0f32).normalize_or_zero();
    let right = transform.right().with_y(0f32).normalize_or_zero();

    let mut dir = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        dir += forward;
    }
    if keys.pressed(KeyCode::KeyS) {
        dir -= forward;
    }
    if keys.pressed(KeyCode::KeyD) {
        dir += right;
    }
    if keys.pressed(KeyCode::KeyA) {
        dir -= right;
    }
    if keys.pressed(KeyCode::Space) {
        dir += Vec3::Y;
    }
    if keys.pressed(KeyCode::ShiftLeft) {
        dir -= Vec3::Y;
    }

    let target_velocity = dir.normalize_or_zero() * speed;
    let delta_time = time.delta_secs();

    cam.velocity.smooth_nudge(&target_velocity, smoothing, delta_time);
    transform.translation += cam.velocity;
}

fn startgame_button() -> impl Bundle {
    (
        Button,
        RRStartgameButton,
        Hovered::default(),
        Node {
            width: percent(100),
            height: px(65),
            border: UiRect::all(px(5)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect { left: Val::Px(50f32), right: Val::Px(50f32), top: Val::Px(20f32), bottom: Val::Px(0f32) },
            ..default()
        },
        BorderColor::all(Color::BLACK),
        BackgroundColor(Color::WHITE),
        children![(
            Text::new("Start Game"),
            TextFont {
                font_size: FontSize::Px(33.0),
                ..default()
            },
            TextColor(Color::BLACK),
        )],
    )
}

fn settings_button() -> impl Bundle {
    (
        Button,
        RRSettingsButton,
        Hovered::default(),
        Node {
            width: percent(100),
            height: px(65),
            border: UiRect::all(px(5)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect { left: Val::Px(50f32), right: Val::Px(50f32), top: Val::Px(20f32), bottom: Val::Px(0f32) },
            ..default()
        },
        BorderColor::all(Color::BLACK),
        BackgroundColor(Color::WHITE),
        children![(
            Text::new("Settings"),
            TextFont {
                font_size: FontSize::Px(33.0),
                ..default()
            },
            TextColor(Color::BLACK),
        )],
    )
}

fn camera_info_text() -> impl Bundle {
    (
        Text::new("No camera found"),
        RRCameraPosInfo,
        TextFont {
            font_size: FontSize::Px(15f32),
            ..default()
        },
        TextColor(Color::BLACK),
        Node {
            position_type: PositionType::Absolute,
            bottom : px(5),
            left : px(5),
            ..default()
        }
    )
}

fn setup_homescreen_buttons(mut commands: Commands) {
    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            align_content: AlignContent::Start,
            justify_content: JustifyContent::Start,
            flex_wrap: FlexWrap::Wrap,
            ..default()
        },
        children![
            (
                startgame_button(),
                observe(|_activate: On<Activate>, mut next: ResMut<NextState<AppState>>| {
                    next.set(AppState::Lobby);
                }),
            ),
            (
                settings_button(),
                observe(|_activate: On<Activate>, mut next: ResMut<NextState<AppState>>| {
                    next.set(AppState::Settings);
                }),
            ),
            (
                camera_info_text()
            )
        ],
        DespawnOnExit(AppState::Homescreen),
    ));
}

fn update_startgame_button_appearance(
    mut buttons: Query
        <
        (
            &Hovered,
            Has<Pressed>,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children
        ),
        (
            With<Button>,
            With<RRStartgameButton>,
        )
        >,
    mut text_query: Query<(&mut Text, &mut TextColor)>
)
{
    for (hovered, pressed, mut bg_color, mut border_color, children) in &mut buttons {
        let Ok((mut text, mut color)) = text_query.get_mut(children[0]) else {
            continue;
        };
        match (hovered.get(), pressed) {
            (_, true) => {
                **text = "Loading...".to_string();
                **color = Color::WHITE;
                *bg_color = BackgroundColor(Color::BLACK);
                border_color.set_all(Color::WHITE);
            },
            (true, false) => {
                **color = Color::WHITE;
                *bg_color = BackgroundColor(Color::hsl(0f32, 0f32, 0.55f32));
                border_color.set_all(Color::BLACK);
            },
            (false, false) => {
                **text = "Start Game".to_string();
                **color = Color::BLACK;
                *bg_color = BackgroundColor(Color::WHITE);
                border_color.set_all(Color::BLACK);
            }
        }
    }
}

fn update_settings_button_appearance(
    mut buttons: Query
        <
        (
            &Hovered,
            Has<Pressed>,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children
        ),
        (
            With<Button>,
            With<RRSettingsButton>
        ),
        >,
    mut text_query: Query<(&mut Text, &mut TextColor)>
)
{
    for (hovered, pressed, mut bg_color, mut border_color, children) in &mut buttons {
        let Ok((mut text, mut color)) = text_query.get_mut(children[0]) else {
            continue;
        };
        match (hovered.get(), pressed) {
            (_, true) => {
                **text = "Loading...".to_string();
                **color = Color::WHITE;
                *bg_color = BackgroundColor(Color::BLACK);
                border_color.set_all(Color::WHITE);
            },
            (true, false) => {
                **color = Color::WHITE;
                *bg_color = BackgroundColor(Color::hsl(0f32, 0f32, 0.55f32));
                border_color.set_all(Color::BLACK);
            },
            (false, false) => {
                **text = "Settings".to_string();
                **color = Color::BLACK;
                *bg_color = BackgroundColor(Color::WHITE);
                border_color.set_all(Color::BLACK);
            }
        }
    }
}

fn update_camera_info
(
    mut info: Single<&mut Text, With<RRCameraPosInfo>>,
    camera: Single<&Transform, With<HomescreenCamera>>
)
{
    let pos = camera.translation;
    info.0 = format!("Camera position: X {:.2} Y {:.2} Z {:.2}", pos.x, pos.y, pos.z);
}

fn test_3d
(
    mut commands: Commands,
    assets: Res<AssetServer>,
)
{
    commands.spawn((
        WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("test.glb"))),
        Transform::from_xyz(0f32, 0f32, 0f32),
        DespawnOnExit(AppState::Homescreen),
    ));

    commands.spawn((
        DirectionalLight {
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(4f32, 8f32, 4f32).looking_at(Vec3::ZERO, Vec3::Y),
        DespawnOnExit(AppState::Homescreen),
    ));
}
