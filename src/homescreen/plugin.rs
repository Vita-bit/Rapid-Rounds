use bevy::prelude::*;
use crate::session::states::*;
use bevy::picking::hover::*;
use bevy::ui::Pressed;
use bevy::ui_widgets::{Button, Activate, observe};

pub struct HomescreenPlugin;

#[derive(Component)]
struct RRStartgameButton;

#[derive(Component)]
struct RRSettingsButton;

impl Plugin for HomescreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Homescreen), (setup_homescreen, setup_homescreen_buttons).chain())
            .add_systems(Update, (homescreen_input, update_startgame_button_appearance, update_settings_button_appearance).run_if(in_state(AppState::Homescreen)));
    }
}

fn setup_homescreen(mut commands: Commands) {
    commands.insert_resource(ClearColor(Color::hsl(0f32, 0f32, 1f32)));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0f32, 5f32, 10f32).looking_at(Vec3::ZERO, Vec3::Y),
        DespawnOnExit(AppState::Homescreen),
    ));
}

fn homescreen_input(keys: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<AppState>>) {
    if keys.just_pressed(KeyCode::Digit1) {
        next.set(AppState::Lobby);
    }
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
            border_radius: BorderRadius::all(Val::Px(20f32)),
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
            border_radius: BorderRadius::all(Val::Px(20f32)),
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
                **text = "Hovered".to_string();
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
                **text = "Hovered".to_string();
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
