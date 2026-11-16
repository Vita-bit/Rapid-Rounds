use bevy::prelude::*;
use crate::session::states::*;

pub struct HomescreenPlugin;

impl Plugin for HomescreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Homescreen), setup_homescreen)
            .add_systems(Update, homescreen_input.run_if(in_state(AppState::Homescreen)));
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
