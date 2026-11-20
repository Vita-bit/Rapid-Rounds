use bevy::prelude::*;
use rapid_rounds::session::game_session::GamePlugin;

fn main() {
    App::new().add_plugins(GamePlugin).run();
}
