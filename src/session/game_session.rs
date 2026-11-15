use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy::window::*;

pub struct Game {
    app: App
}

impl Game {
    pub fn new() -> Game {
        let app = App::new();
        Game {app}
    }

    pub fn init(&mut self) {
        self.app
            .add_plugins(DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Self::create_game_window()),
                    ..default()
                }))
            .add_systems(Update, Self::change_window_mode.run_if(input_just_pressed(KeyCode::F11)))
            .run();
    }

    pub fn create_game_window() -> Window {
        Window {
            title: "Game".to_string(),
            mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
            resizable: true,
            ..default()
        }
    }

    pub fn change_window_mode(mut window: Single<&mut Window, With<PrimaryWindow>>) {
        window.mode = match window.mode {
            WindowMode::BorderlessFullscreen(MonitorSelection::Primary) => WindowMode::Windowed,
            _ => WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
        };
    }
}
