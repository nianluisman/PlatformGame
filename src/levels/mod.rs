mod level_one;

use bevy::prelude::*;

use crate::render::PLATFORM_COLOR;

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(level_one::definition())
            .add_systems(Startup, spawn_platforms);
    }
}

/// Everything needed to construct one static level. Gameplay systems read
/// this data but do not decide where level objects belong.
#[derive(Resource)]
pub struct LevelDefinition {
    pub player_start: Vec3,
    pub platforms: Vec<PlatformDefinition>,
    pub spikes: Vec<SpikeDefinition>,
}

pub struct PlatformDefinition {
    pub position: Vec2,
    pub size: Vec2,
}

pub struct SpikeDefinition {
    pub position: Vec2,
    pub size: Vec2,
}

/// Marker for solid, axis-aligned level geometry.
#[derive(Component)]
pub struct Platform;

fn spawn_platforms(mut commands: Commands, level: Res<LevelDefinition>) {
    for platform in &level.platforms {
        commands.spawn((
            Platform,
            Sprite::from_color(PLATFORM_COLOR, platform.size),
            Transform::from_xyz(platform.position.x, platform.position.y, 0.0),
        ));
    }
}
