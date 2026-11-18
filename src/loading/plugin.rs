use bevy::prelude::*;
use crate::session::states::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Loading), setup_loading);
    }
}

fn setup_loading(mut commands: Commands) {
    commands.insert_resource(ClearColor(Color::hsl(0f32, 0f32, 1f32)));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0f32, 5f32, 10f32).looking_at(Vec3::ZERO, Vec3::Y),
        DespawnOnExit(AppState::Loading),
    ));
}
