use bevy::prelude::*;

use crate::homescreen::plugin::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(HomescreenSubstates::Menu), setup_menu);
    }
}

fn setup_menu(mut commands: Commands) {
    commands.insert_resource(ClearColor(Color::hsl(0f32, 0f32, 1f32)));
}
