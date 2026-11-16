use bevy::prelude::*;
use project::session::game_session::GamePlugin;

fn main() {
    App::new().add_plugins(GamePlugin).run();
}
