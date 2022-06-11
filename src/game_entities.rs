use super::utils;
use bevy::prelude::*;
use bevy_rapier2d::prelude::{ActiveEvents, Ccd, Collider, RigidBody, LockedAxes};
use std::collections::HashMap;

pub struct GoalEvent {
    pub ball_id: u32,
    pub team: Team,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Team {
    Player = 0,
    AI = 1,
}

impl Opposite for Team {
    fn opposite(&self) -> Self {
        match self {
            Team::Player => Team::AI,
            Team::AI => Team::Player,
        }
    }
}

pub trait Opposite {
    fn opposite(&self) -> Self;
}

pub trait Reset {
    fn reset(&mut self);
}

#[derive(Clone, Component, Default)]
pub struct Speed {
    pub initial: f32,
    pub current: f32,
}

impl Reset for Speed {
    fn reset(&mut self) {
        self.current = self.initial;
    }
}

#[derive(Clone, Component, Default)]
pub struct MovementDirection(pub Vec3);

impl MovementDirection {
    pub fn set_random_horizontal(&mut self) {
        self.0 = Vec3::new(utils::rand_sign(), 0.0, 0.0);
    }

    pub fn from_random_horizontal() -> Self {
        let mut dir = MovementDirection(Vec3::ZERO);
        dir.set_random_horizontal();
        dir
    }
}

#[derive(Clone, Component, Default)]
pub struct Paddle;

#[derive(Clone, Component, Default)]
pub struct PlayerPaddle;

#[derive(Clone, Component, Default)]
pub struct AIPaddle;

#[derive(Clone, Component)]
pub struct Ball {
    pub speed_multiplier: f32,
}

#[derive(Clone, Component, Default)]
pub struct Bounds;

#[derive(Clone, Component)]
pub struct Goal {
    pub team: Team,
}

#[derive(Bundle)]
pub struct PaddleBundle {
    pub paddle: Paddle,
    pub speed: Speed,
    pub mov_dir: MovementDirection,
    #[bundle]
    pub sprite: SpriteBundle,
    pub rb: RigidBody,
    pub collider: Collider,
    pub coll_events: ActiveEvents,
    pub locked_axes: LockedAxes,
}

#[derive(Bundle)]
pub struct BallBundle {
    pub ball: Ball,
    pub speed: Speed,
    pub mov_dir: MovementDirection,
    #[bundle]
    pub sprite: SpriteBundle,
    pub rb: RigidBody,
    pub collider: Collider,
    pub coll_events: ActiveEvents,
    pub locked_axes: LockedAxes,
    pub ccd: Ccd,
}

#[derive(Bundle)]
pub struct BoundsBundle {
    pub bounds: Bounds,
    #[bundle]
    pub sprite: SpriteBundle,
    pub rb: RigidBody,
    pub collider: Collider,
    pub coll_events: ActiveEvents,
}

#[derive(Clone, Component, Default)]
pub struct MatchScore {
    pub score: HashMap<Team, i32>,
}
