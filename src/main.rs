mod enemies;
mod game;
mod game_mechanics;
mod levels;
mod render;
mod utility;

use bevy::prelude::*;
use game::GamePlugin;

/// Application entry point. Game features are composed through plugins.
fn main() {
    App::new().add_plugins((DefaultPlugins, GamePlugin)).run();
}
