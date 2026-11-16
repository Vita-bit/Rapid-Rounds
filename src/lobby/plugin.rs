use bevy::prelude::*;
use crate::session::states::*;

pub struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Lobby), setup_lobby)
            .add_systems(Update, lobby_input.run_if(in_state(AppState::Lobby)));
    }
}

fn setup_lobby(mut commands: Commands) {
    commands.insert_resource(ClearColor(Color::hsl(0.1f32, 0.4f32, 0.1f32)));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0f32, 5f32, 10f32).looking_at(Vec3::ZERO, Vec3::Y),
        DespawnOnExit(AppState::Lobby),
    ));
}

fn lobby_input(keys: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<AppState>>) {
    if keys.just_pressed(KeyCode::Escape) {
        next.set(AppState::Homescreen);
    }
}
