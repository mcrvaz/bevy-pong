use bevy::{prelude::*, sprite::collide_aabb};
use rand::prelude::*;
use std::{iter::Sum, ops::Div};

pub fn collision_to_direction(collision: collide_aabb::Collision) -> Vec2 {
    match collision {
        collide_aabb::Collision::Left => Vec2::new(-1.0, 0.0),
        collide_aabb::Collision::Right => Vec2::new(1.0, 0.0),
        collide_aabb::Collision::Top => Vec2::new(0.0, 1.0),
        collide_aabb::Collision::Bottom => Vec2::new(0.0, -1.0),
        collide_aabb::Collision::Inside => Vec2::ZERO,
    }
}

pub fn avg<'a, T>(values: &'a [T]) -> T
where
    T: Sum<&'a T> + Div<f32, Output = T>,
{
    values.iter().sum::<T>() / (values.len() as f32)
}

pub fn v2_to_v3(v2: Vec2) -> Vec3 {
    v2.extend(0.0)
}

pub fn rand_sign() -> f32 {
    if random::<f32>() >= 0.5 {
        1.0
    } else {
        -1.0
    }
}
