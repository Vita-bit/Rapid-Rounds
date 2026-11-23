use bevy::prelude::*;

use crate::homescreen::plugin::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(HomescreenSubstates::Loading), (setup_loading, setup_loading_ui).chain());
    }
}

fn setup_loading(mut commands: Commands) {
    commands.insert_resource(ClearColor(Color::hsl(0f32, 0f32, 0f32)));
}

fn loading_text() -> impl Bundle {
    (
        Text::new("Loading..."),
        TextColor(Color::WHITE),
        TextFont {
            font_size: FontSize::Px(60f32),
            ..default()
        },
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_content: AlignContent::Center,
            ..default()
        }
    )
}

fn setup_loading_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_content: AlignContent::Center,
            ..default()
        },
        children![
            (
                loading_text()
            )
        ]
    ))
}