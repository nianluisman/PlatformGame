mod spike;

use bevy::prelude::*;
use spike::SpikePlugin;

/// Entry point for traps and enemies. Future moving enemies can be registered
/// here while sharing the same player damage/respawn rules.
pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SpikePlugin);
    }
}
