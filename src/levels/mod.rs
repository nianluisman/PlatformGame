use std::path::PathBuf;

use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{TiledPlugin, tiled};

use crate::render::PLATFORM_COLOR;

const LEVEL_ASSET: &str = "maps/level_one.tmx";

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TiledPlugin::default())
            .add_systems(PreStartup, load_level_definition)
            .add_systems(Startup, spawn_platforms);
    }
}

#[derive(Resource)]
pub struct LevelDefinition {
    pub player_start: Vec3,
    pub platforms: Vec<PlatformDefinition>,
    pub spikes: Vec<SpikeDefinition>,
    pub enemies: Vec<EnemyDefinition>,
}

pub struct PlatformDefinition {
    pub position: Vec2,
    pub size: Vec2,
}

pub struct SpikeDefinition {
    pub position: Vec2,
    pub size: Vec2,
}

pub struct EnemyDefinition {
    pub position: Vec2,
    pub size: Vec2,
}

#[derive(Component)]
pub struct Platform;

fn load_level_definition(mut commands: Commands) {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join(LEVEL_ASSET);
    let mut loader = tiled::Loader::new();
    let map = loader
        .load_tmx_map(&path)
        .unwrap_or_else(|error| panic!("could not load {}: {error}", path.display()));

    let map_size = UVec2::new(map.width, map.height);
    let tile_size = Vec2::new(map.tile_width as f32, map.tile_height as f32);
    let mut level = LevelDefinition {
        player_start: Vec3::new(0.0, 0.0, 1.0),
        platforms: Vec::new(),
        spikes: Vec::new(),
        enemies: Vec::new(),
    };

    for layer in map.layers() {
        let layer_name = layer.name.as_str();
        let Some(tiles) = layer.as_tile_layer() else {
            continue;
        };

        match layer_name {
            "Collisions" => {
                for y in 0..map.height as i32 {
                    let mut x = 0;
                    while x < map.width as i32 {
                        if tiles.get_tile(x, y).is_none() {
                            x += 1;
                            continue;
                        }

                        let start_x = x;
                        while x < map.width as i32 && tiles.get_tile(x, y).is_some() {
                            x += 1;
                        }
                        let run = (x - start_x) as u32;
                        level.platforms.push(PlatformDefinition {
                            position: grid_rectangle_center(
                                UVec2::new(start_x as u32, y as u32),
                                UVec2::new(run, 1),
                                map_size,
                                tile_size,
                            ),
                            size: Vec2::new(run as f32 * tile_size.x, tile_size.y),
                        });
                    }
                }
            }
            "Hazards" => {
                for y in 0..map.height as i32 {
                    for x in 0..map.width as i32 {
                        if tiles.get_tile(x, y).is_some() {
                            level.spikes.push(SpikeDefinition {
                                position: grid_cell_center(
                                    UVec2::new(x as u32, y as u32),
                                    map_size,
                                    tile_size,
                                ),
                                size: tile_size,
                            });
                        }
                    }
                }
            }
            "Spawns" => {
                for y in 0..map.height as i32 {
                    for x in 0..map.width as i32 {
                        if tiles.get_tile(x, y).is_some() {
                            level.player_start = grid_cell_center(
                                UVec2::new(x as u32, y as u32),
                                map_size,
                                tile_size,
                            )
                            .extend(1.0);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    // Place the first moving enemy on the long starting platform. More enemy
    // definitions can be added here or loaded from a dedicated Tiled layer.
    let enemy_size = Vec2::new(42.0, 58.0);
    let mut enemy_position =
        grid_cell_center(UVec2::new(20, 16), map_size, tile_size);

    // Keep the enemy's bottom aligned with the platform.
    enemy_position.y += (enemy_size.y - tile_size.y) * 0.5;

    level.enemies.push(EnemyDefinition {
        position: enemy_position,
        size: enemy_size,
    });

    commands.insert_resource(level);
}

fn spawn_platforms(mut commands: Commands, level: Res<LevelDefinition>) {
    for platform in &level.platforms {
        commands.spawn((
            Platform,
            Sprite::from_color(PLATFORM_COLOR, platform.size),
            Transform::from_xyz(platform.position.x, platform.position.y, 0.0),
        ));
    }
}

/// Converts a Tiled `(column, row)` index to a centered Bevy position.
fn grid_cell_center(cell: UVec2, map_size: UVec2, tile_size: Vec2) -> Vec2 {
    let map_pixels = map_size.as_vec2() * tile_size;
    Vec2::new(
        (cell.x as f32 + 0.5) * tile_size.x - map_pixels.x * 0.5,
        map_pixels.y * 0.5 - (cell.y as f32 + 0.5) * tile_size.y,
    )
}

fn grid_rectangle_center(
    start: UVec2,
    size_in_tiles: UVec2,
    map_size: UVec2,
    tile_size: Vec2,
) -> Vec2 {
    grid_cell_center(start, map_size, tile_size)
        + Vec2::new(
            (size_in_tiles.x - 1) as f32 * tile_size.x * 0.5,
            -((size_in_tiles.y - 1) as f32) * tile_size.y * 0.5,
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_grid_index_to_centered_world() {
        let center = grid_cell_center(UVec2::new(19, 17), UVec2::new(40, 20), Vec2::splat(32.0));
        assert_eq!(center, Vec2::new(-16.0, -240.0));
    }
}
