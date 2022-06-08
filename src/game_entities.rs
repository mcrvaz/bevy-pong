use bevy::prelude::*;

#[derive(Clone, Component, Default)]
pub struct Speed(pub f32);

#[derive(Clone, Component, Default)]
pub struct MovementDirection(pub Vec3);

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
impl Default for Ball {
    fn default() -> Self {
        Self {
            speed_multiplier: 1.0,
        }
    }
}

#[derive(Clone, Component, Default)]
pub struct Bounds;

#[derive(Bundle, Default)]
pub struct PaddleBundle {
    pub paddle: Paddle,
    pub speed: Speed,
    pub mov_dir: MovementDirection,
    #[bundle]
    pub sprite: SpriteBundle,
}

#[derive(Bundle, Default)]
pub struct BallBundle {
    pub ball: Ball,
    pub speed: Speed,
    pub mov_dir: MovementDirection,
    #[bundle]
    pub sprite: SpriteBundle,
}

#[derive(Bundle, Default)]
pub struct BoundsBundle {
    pub bounds: Bounds,
    #[bundle]
    pub sprite: SpriteBundle,
}
