use bevy::prelude::*;
use crate::homescreen::plugin::*;

pub struct ChoosingGamePlugin;

impl Plugin for ChoosingGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(HomescreenSubstates::ChoosingGame), setup_choosing_game)
            .add_systems(Update, choosing_game_input.run_if(in_state(HomescreenSubstates::ChoosingGame)));
    }
}

fn setup_choosing_game(mut commands: Commands) {
    commands.insert_resource(ClearColor(Color::hsl(100f32, 1f32, 0.5f32)));
}

fn choosing_game_input(keys: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<HomescreenSubstates>>) {
    if keys.just_pressed(KeyCode::Escape) {
        next.set(HomescreenSubstates::Menu);
    }
}
