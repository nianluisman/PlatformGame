use bevy::prelude::*;

use crate::{
    enemies::EnemiesPlugin, game_mechanics::GameMechanicsPlugin, levels::LevelsPlugin,
    render::RenderPlugin,
};

/// Composes all major parts of the game in one place.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RenderPlugin,
            LevelsPlugin,
            GameMechanicsPlugin,
            EnemiesPlugin,
        ));
    }
}
