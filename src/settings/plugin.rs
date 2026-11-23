use bevy::prelude::*;

use crate::session::states::*;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Settings), setup_settings)
            .add_systems(Update, settings_input.run_if(in_state(AppState::Settings)));
    }
}

fn setup_settings(mut commands: Commands) {
    commands.insert_resource(ClearColor(Color::hsl(110f32, 1f32, 0.27f32)));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0f32, 5f32, 10f32).looking_at(Vec3::ZERO, Vec3::Y),
        DespawnOnExit(AppState::Settings),
    ));
}

fn settings_input(keys: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<AppState>>) {
    if keys.just_pressed(KeyCode::Escape) {
        next.set(AppState::Homescreen);
    }
}
