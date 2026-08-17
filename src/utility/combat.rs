use std::time::Duration;

use bevy::prelude::*;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum Team {
    Player,
    Enemy,
}

#[derive(Component)]
pub struct Facing(pub f32);

#[derive(Component)]
pub struct Attacker {
    pub cooldown: Timer,
    pub slash_size: Vec2,
    pub slash_offset: f32,
    pub slash_duration: Duration,
}

impl Attacker {
    pub fn new(
        cooldown: Duration,
        slash_size: Vec2,
        slash_offset: f32,
        slash_duration: Duration,
    ) -> Self {
        let mut cooldown = Timer::new(cooldown, TimerMode::Once);
        cooldown.finish();
        Self {
            cooldown,
            slash_size,
            slash_offset,
            slash_duration,
        }
    }
}

#[derive(Component)]
pub struct AttackRequest;

#[derive(Component)]
pub struct Slash {
    pub team: Team,
    pub size: Vec2,
    pub lifetime: Timer,
}
