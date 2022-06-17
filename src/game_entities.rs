use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
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

#[derive(Clone, Component)]
pub struct Paddle {
    pub speed: f32,
}

#[derive(Clone, Component, Default)]
pub struct PlayerPaddle;

#[derive(Clone, Component, Default)]
pub struct AIPaddle;

#[derive(Clone, Component)]
pub struct Ball {
    pub initial_speed: f32,
    pub speed_multiplier: f32,
    pub max_speed: Vec2,
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
    #[bundle]
    pub sprite: SpriteBundle,
    pub rb: RigidBody,
    pub collider: Collider,
    pub coll_events: ActiveEvents,
    pub ccd: Ccd,
}

#[derive(Bundle)]
pub struct BoundsBundle {
    pub bounds: Bounds,
    #[bundle]
    pub sprite: SpriteBundle,
    pub collider: Collider,
    pub coll_events: ActiveEvents,
}

#[derive(Clone, Component, Default)]
pub struct MatchScore {
    pub score: HashMap<Team, i32>,
}

pub struct BallLaunchDelay(pub Timer);
pub struct BallLaunch;

#[derive(Component)]
pub struct ScoreText {
    pub team: Team,
}

#[derive(Component)]
pub struct BallLaunchTimerText;
