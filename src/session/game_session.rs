use bevy::input::common_conditions::*;
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
            .insert_resource(ClearColor(Color::hsl(0f32, 0f32, 1f32)))
            .add_systems(Update, Self::change_window_mode.run_if(input_just_pressed(KeyCode::F11)))
            .add_systems(Startup, Self::spawn_camera)
            .run();
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

    fn spawn_camera(mut commands: Commands) {
        commands.spawn(Camera2d);
    }
}
