use bevy::input::common_conditions::*;
use bevy::prelude::*;
use bevy::window::*;
use crate::homescreen::plugin::HomescreenPlugin;
use crate::lobby::plugin::LobbyPlugin;
use crate::session::states::AppState;
use crate::settings::plugin::SettingsPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(create_game_window()),
                    ..default()
                }))
            .init_state::<AppState>()
            .add_plugins((HomescreenPlugin, LobbyPlugin, SettingsPlugin))
            .add_systems(Update, change_window_mode.run_if(input_just_pressed(KeyCode::F11)));
    }
}

fn create_game_window() -> Window {
    Window {
        title: "Game".to_string(),
        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
        resizable: true,
        ..default()
    }
}

fn change_window_mode(mut window: Single<&mut Window, With<PrimaryWindow>>) {
    window.mode = match window.mode {
        WindowMode::BorderlessFullscreen(MonitorSelection::Primary) => WindowMode::Windowed,
        _ => WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
    };
}
