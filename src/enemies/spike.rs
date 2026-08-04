use bevy::prelude::*;

use crate::{
    game_mechanics::{Player, PlayerSize, RespawnPlayer},
    levels::LevelDefinition,
    render::SPIKE_COLOR,
};

pub struct SpikePlugin;

impl Plugin for SpikePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spikes)
            .add_systems(Update, detect_spike_collision);
    }
}

#[derive(Component)]
struct Spike {
    size: Vec2,
}

fn spawn_spikes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    level: Res<LevelDefinition>,
) {
    for spike in &level.spikes {
        commands.spawn((
            Spike { size: spike.size },
            Mesh2d(meshes.add(Triangle2d::new(
                Vec2::new(-spike.size.x * 0.5, -spike.size.y * 0.5),
                Vec2::new(spike.size.x * 0.5, -spike.size.y * 0.5),
                Vec2::new(0.0, spike.size.y * 0.5),
            ))),
            MeshMaterial2d(materials.add(SPIKE_COLOR)),
            Transform::from_xyz(spike.position.x, spike.position.y, 1.0),
        ));
    }
}

fn detect_spike_collision(
    player: Query<(&Transform, &PlayerSize), With<Player>>,
    spikes: Query<(&Transform, &Spike)>,
    mut respawn: ResMut<RespawnPlayer>,
) {
    let Ok((player_transform, player_size)) = player.single() else {
        return;
    };

    let player_position = player_transform.translation.truncate();
    for (spike_transform, spike) in &spikes {
        if overlaps(
            player_position,
            player_size.0,
            spike_transform.translation.truncate(),
            spike.size,
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
    fn separated_colliders_do_not_overlap() {
        assert!(!overlaps(
            Vec2::ZERO,
            Vec2::splat(10.0),
            Vec2::new(11.0, 0.0),
            Vec2::splat(10.0),
        ));
    }
}
