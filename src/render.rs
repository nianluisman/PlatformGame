use bevy::prelude::*;

pub const PLAYER_COLOR: Color = Color::srgb(0.95, 0.65, 0.20);
pub const PLATFORM_COLOR: Color = Color::srgb(0.25, 0.65, 0.38);
pub const SPIKE_COLOR: Color = Color::srgb(0.90, 0.18, 0.16);
pub const MOVING_ENEMY: Color = Color::srgb(0.90, 0.0, 0.0);
/// Owns global presentation settings. Sprites, animation and lighting can be
/// added under this module as the visual side of the game grows.
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(0.08, 0.12, 0.20)));
    }
}
