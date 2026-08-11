use bevy::prelude::*;

use crate::{
    game_mechanics::{Player, PlayerSize, RespawnPlayer},
    levels::{LevelDefinition, Platform},
    render::MOVING_ENEMY,
};

const ENEMY_SPEED: f32 = 110.0;
const FOOT_PROBE_DEPTH: f32 = 4.0;

pub struct MovingEnemyPlugin;

impl Plugin for MovingEnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemies)
            .add_systems(Update, (move_enemies, detect_enemy_collision).chain());
    }
}

#[derive(Component)]
struct MovingEnemy {
    size: Vec2,
    direction: f32,
}

fn spawn_enemies(mut commands: Commands, level: Res<LevelDefinition>) {
    for enemy in &level.enemies {
        commands.spawn((
            MovingEnemy {
                size: enemy.size,
                direction: 1.0,
            },
            Sprite::from_color(MOVING_ENEMY, enemy.size),
            Transform::from_xyz(enemy.position.x, enemy.position.y, 1.0),
        ));
    }
}

fn move_enemies(
    time: Res<Time>,
    mut enemies: Query<(&mut Transform, &mut MovingEnemy)>,
    platforms: Query<(&Transform, &Sprite), (With<Platform>, Without<MovingEnemy>)>,
) {
    let dt = time.delta_secs().min(1.0 / 20.0);

    for (mut transform, mut enemy) in &mut enemies {
        let next_x = transform.translation.x + enemy.direction * ENEMY_SPEED * dt;
        let probe = Vec2::new(
            next_x + enemy.direction * enemy.size.x * 0.5,
            transform.translation.y - enemy.size.y * 0.5 - FOOT_PROBE_DEPTH,
        );

        if has_platform_under(probe, &platforms) {
            transform.translation.x = next_x;
        } else {
            enemy.direction *= -1.0;
        }
    }
}

fn has_platform_under(
    point: Vec2,
    platforms: &Query<(&Transform, &Sprite), (With<Platform>, Without<MovingEnemy>)>,
) -> bool {
    platforms.iter().any(|(transform, sprite)| {
        let Some(size) = sprite.custom_size else {
            return false;
        };
        let center = transform.translation.truncate();
        let half = size * 0.5;
        point.x >= center.x - half.x
            && point.x <= center.x + half.x
            && point.y >= center.y - half.y
            && point.y <= center.y + half.y
    })
}

fn detect_enemy_collision(
    player: Query<(&Transform, &PlayerSize), With<Player>>,
    enemies: Query<(&Transform, &MovingEnemy)>,
    mut respawn: ResMut<RespawnPlayer>,
) {
    let Ok((player_transform, player_size)) = player.single() else {
        return;
    };

    let player_position = player_transform.translation.truncate();
    for (enemy_transform, enemy) in &enemies {
        if overlaps(
            player_position,
            player_size.0,
            enemy_transform.translation.truncate(),
            enemy.size,
        ) {
            respawn.0 = true;
            return;
        }
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
    fn enemy_collision_uses_both_collider_sizes() {
        assert!(overlaps(
            Vec2::ZERO,
            Vec2::splat(10.0),
            Vec2::new(9.0, 0.0),
            Vec2::splat(10.0),
        ));
        assert!(!overlaps(
            Vec2::ZERO,
            Vec2::splat(10.0),
            Vec2::new(10.0, 0.0),
            Vec2::splat(10.0),
        ));
    }
}
