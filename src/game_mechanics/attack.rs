use bevy::prelude::*;

use crate::{
    game_mechanics::Player,
    utility::combat::{AttackRequest, Attacker, Facing, Slash, Team},
};

const SLASH_COLOR: Color = Color::srgba(1.0, 0.9, 0.3, 0.75);

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                request_player_attack,
                spawn_requested_attacks,
                expire_slashes,
            )
                .chain(),
        );
    }
}

fn request_player_attack(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    player: Query<Entity, With<Player>>,
) {
    if keyboard.just_pressed(KeyCode::KeyJ) || keyboard.just_pressed(KeyCode::KeyF) {
        if let Ok(entity) = player.single() {
            commands.entity(entity).insert(AttackRequest);
        }
    }
}

/// Any entity can attack by owning these shared components and an `AttackRequest`.
fn spawn_requested_attacks(
    mut commands: Commands,
    time: Res<Time>,
    mut attackers: Query<
        (Entity, &Facing, &Team, &mut Attacker, Has<AttackRequest>),
        Without<Slash>,
    >,
) {
    for (entity, facing, team, mut attacker, requested) in &mut attackers {
        attacker.cooldown.tick(time.delta());
        if !requested {
            continue;
        }
        commands.entity(entity).remove::<AttackRequest>();
        if !attacker.cooldown.is_finished() {
            continue;
        }

        let size = attacker.slash_size;
        let duration = attacker.slash_duration;
        let offset = facing.0 * attacker.slash_offset;
        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                Slash {
                    team: *team,
                    size,
                    lifetime: Timer::new(duration, TimerMode::Once),
                },
                Sprite::from_color(SLASH_COLOR, size),
                Transform::from_xyz(offset, 0.0, 1.0),
            ));
        });
        attacker.cooldown.reset();
    }
}

fn expire_slashes(
    mut commands: Commands,
    time: Res<Time>,
    mut slashes: Query<(Entity, &mut Slash)>,
) {
    for (entity, mut slash) in &mut slashes {
        slash.lifetime.tick(time.delta());
        if slash.lifetime.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
