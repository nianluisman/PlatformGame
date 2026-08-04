use bevy::prelude::*;

use super::Player;

const CAMERA_FOLLOW_SPEED: f32 = 5.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, follow_player);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn follow_player(
    time: Res<Time>,
    player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let (Ok(player), Ok(mut camera)) = (player.single(), camera.single_mut()) else {
        return;
    };

    let target = Vec2::new(player.translation.x, player.translation.y.max(0.0));
    let smoothing = 1.0 - (-CAMERA_FOLLOW_SPEED * time.delta_secs()).exp();
    let position = camera.translation.truncate().lerp(target, smoothing);
    camera.translation.x = position.x;
    camera.translation.y = position.y;
}
