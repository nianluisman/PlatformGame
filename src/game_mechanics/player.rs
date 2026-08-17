use bevy::prelude::*;
use std::time::Duration;

use crate::{
    levels::{LevelDefinition, Platform},
    render::PLAYER_COLOR,
    utility::combat::{Attacker, Facing, Team},
};

const PLAYER_SIZE: Vec2 = Vec2::new(42.0, 58.0);
const MOVE_SPEED: f32 = 330.0;
const JUMP_SPEED: f32 = 860.0;
const GRAVITY: f32 = -1900.0;
const MAX_FALL_SPEED: f32 = -900.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RespawnPlayer>()
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (read_player_input, move_player).chain())
            .add_systems(PostUpdate, respawn_player);
    }
}

#[derive(Component)]
pub struct Player;

/// Collider dimensions shared with hazards without exposing player physics.
#[derive(Component)]
pub struct PlayerSize(pub Vec2);

/// Hazards and game rules request a reset through this resource. This keeps
/// them independent from the player's internal velocity and grounded state.
#[derive(Resource, Default)]
pub struct RespawnPlayer(pub bool);

#[derive(Component, Default)]
struct Velocity(Vec2);

#[derive(Component, Default)]
struct Grounded(bool);

fn spawn_player(mut commands: Commands, level: Res<LevelDefinition>) {
    commands.spawn((
        Player,
        Team::Player,
        Facing(1.0),
        Attacker::new(
            Duration::from_millis(280),
            Vec2::new(54.0, 46.0),
            46.0,
            Duration::from_millis(120),
        ),
        PlayerSize(PLAYER_SIZE),
        Velocity::default(),
        Grounded::default(),
        Sprite::from_color(PLAYER_COLOR, PLAYER_SIZE),
        Transform::from_translation(level.player_start),
    ));
}

fn read_player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut Velocity, &Grounded, &mut Facing), With<Player>>,
) {
    let Ok((mut velocity, grounded, mut facing)) = player.single_mut() else {
        return;
    };

    let left = keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft);
    let right = keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight);
    velocity.0.x = (right as i8 - left as i8) as f32 * MOVE_SPEED;
    if velocity.0.x != 0.0 {
        facing.0 = velocity.0.x.signum();
    }

    let jump = keyboard.just_pressed(KeyCode::Space)
        || keyboard.just_pressed(KeyCode::KeyW)
        || keyboard.just_pressed(KeyCode::ArrowUp);
    if jump && grounded.0 {
        velocity.0.y = JUMP_SPEED;
    }
}

fn move_player(
    time: Res<Time>,
    mut respawn: ResMut<RespawnPlayer>,
    mut player: Query<(&mut Transform, &mut Velocity, &mut Grounded), With<Player>>,
    platforms: Query<(&Transform, &Sprite), (With<Platform>, Without<Player>)>,
) {
    let Ok((mut transform, mut velocity, mut grounded)) = player.single_mut() else {
        return;
    };

    let dt = time.delta_secs().min(1.0 / 20.0);
    velocity.0.y = (velocity.0.y + GRAVITY * dt).max(MAX_FALL_SPEED);
    grounded.0 = false;

    transform.translation.x += velocity.0.x * dt;
    resolve_horizontal(&mut transform, &mut velocity, &platforms);
    transform.translation.y += velocity.0.y * dt;
    resolve_vertical(&mut transform, &mut velocity, &mut grounded, &platforms);

    if transform.translation.y < -700.0 {
        respawn.0 = true;
    }
}

fn respawn_player(
    mut request: ResMut<RespawnPlayer>,
    level: Res<LevelDefinition>,
    mut player: Query<(&mut Transform, &mut Velocity, &mut Grounded), With<Player>>,
) {
    if !request.0 {
        return;
    }
    request.0 = false;

    let Ok((mut transform, mut velocity, mut grounded)) = player.single_mut() else {
        return;
    };
    transform.translation = level.player_start;
    velocity.0 = Vec2::ZERO;
    grounded.0 = false;
}

fn resolve_horizontal(
    player: &mut Transform,
    velocity: &mut Velocity,
    platforms: &Query<(&Transform, &Sprite), (With<Platform>, Without<Player>)>,
) {
    for (platform, sprite) in platforms {
        let Some(size) = sprite.custom_size else {
            continue;
        };
        if !overlaps(
            player.translation.truncate(),
            PLAYER_SIZE,
            platform.translation.truncate(),
            size,
        ) {
            continue;
        }
        let half_widths = (PLAYER_SIZE.x + size.x) * 0.5;
        player.translation.x = platform.translation.x
            + if velocity.0.x > 0.0 {
                -half_widths
            } else {
                half_widths
            };
        velocity.0.x = 0.0;
    }
}

fn resolve_vertical(
    player: &mut Transform,
    velocity: &mut Velocity,
    grounded: &mut Grounded,
    platforms: &Query<(&Transform, &Sprite), (With<Platform>, Without<Player>)>,
) {
    for (platform, sprite) in platforms {
        let Some(size) = sprite.custom_size else {
            continue;
        };
        if !overlaps(
            player.translation.truncate(),
            PLAYER_SIZE,
            platform.translation.truncate(),
            size,
        ) {
            continue;
        }
        let half_heights = (PLAYER_SIZE.y + size.y) * 0.5;
        if velocity.0.y <= 0.0 {
            player.translation.y = platform.translation.y + half_heights;
            grounded.0 = true;
        } else {
            player.translation.y = platform.translation.y - half_heights;
        }
        velocity.0.y = 0.0;
    }
}

fn overlaps(a_position: Vec2, a_size: Vec2, b_position: Vec2, b_size: Vec2) -> bool {
    let distance = (a_position - b_position).abs();
    distance.x < (a_size.x + b_size.x) * 0.5 && distance.y < (a_size.y + b_size.y) * 0.5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aabb_overlap_detects_intersection() {
        assert!(overlaps(
            Vec2::ZERO,
            Vec2::splat(10.0),
            Vec2::new(9.0, 0.0),
            Vec2::splat(10.0)
        ));
        assert!(!overlaps(
            Vec2::ZERO,
            Vec2::splat(10.0),
            Vec2::new(10.0, 0.0),
            Vec2::splat(10.0)
        ));
    }
}
