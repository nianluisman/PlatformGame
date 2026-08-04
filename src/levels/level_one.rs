use bevy::prelude::*;

use super::{LevelDefinition, PlatformDefinition, SpikeDefinition};

/// Edit this function to build level one.
///
/// Positions use the center of an object. For a spike resting on a platform,
/// place its center at: platform top + half the spike height.
pub fn definition() -> LevelDefinition {
    LevelDefinition {
        player_start: Vec3::new(-430.0, -170.0, 1.0),

        // Surfaces the player can stand and jump on.
        platforms: vec![
            platform(0.0, -260.0, 1100.0, 60.0),
            platform(-300.0, -130.0, 220.0, 35.0),
            platform(20.0, -20.0, 230.0, 35.0),
            platform(330.0, 100.0, 190.0, 35.0),
            platform(600.0, -40.0, 180.0, 35.0),
        ],

        // Enemy and trap placement for this level.
        spikes: vec![
            spike(-80.0, -211.0, 50.0, 38.0),
            spike(180.0, -211.0, 50.0, 38.0),
            spike(470.0, -211.0, 50.0, 38.0),
        ],
    }
}

fn platform(x: f32, y: f32, width: f32, height: f32) -> PlatformDefinition {
    PlatformDefinition {
        position: Vec2::new(x, y),
        size: Vec2::new(width, height),
    }
}

fn spike(x: f32, y: f32, width: f32, height: f32) -> SpikeDefinition {
    SpikeDefinition {
        position: Vec2::new(x, y),
        size: Vec2::new(width, height),
    }
}
