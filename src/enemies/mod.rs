mod moving_enemy;
mod spike;

use bevy::prelude::*;
use moving_enemy::MovingEnemyPlugin;
use spike::SpikePlugin;

/// Entry point for traps and enemies. Future moving enemies can be registered
/// here while sharing the same player damage/respawn rules.
pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SpikePlugin, MovingEnemyPlugin));
    }
}
