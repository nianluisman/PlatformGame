mod camera;
mod player;

use bevy::prelude::*;
use camera::CameraPlugin;
use player::PlayerPlugin;

pub use player::{Player, PlayerSize, RespawnPlayer};

/// Groups mechanics such as input, movement, physics and camera behavior.
pub struct GameMechanicsPlugin;

impl Plugin for GameMechanicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerPlugin, CameraPlugin));
    }
}
