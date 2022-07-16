use bevy::prelude::*;
use rand::prelude::*;
use std::{iter::Sum, ops::Div};

#[allow(dead_code)]
pub fn avg<'a, T>(values: &'a [T]) -> T
where
    T: Sum<&'a T> + Div<f32, Output = T>,
{
    values.iter().sum::<T>() / (values.len() as f32)
}

pub fn rand_sign() -> f32 {
    if random::<f32>() >= 0.5 {
        1.0
    } else {
        -1.0
    }
}

pub fn random_horizontal() -> Vec2 {
    Vec2::new(rand_sign(), 0.0)
}

pub fn approx_eq(a: f32, b: f32, margin: f32) -> bool {
    (a - b).abs() <= margin
}

pub fn inverse_lerp(a: f32, b: f32, value: f32) -> f32 {
    if a != b {
        clamp_01((value - a) / (b - a))
    } else {
        0.0
    }
}

pub fn clamp_01(v: f32) -> f32 {
    if v < 0.0 {
        0.0
    } else if v > 1.0 {
        1.0
    } else {
        v
    }
}

pub fn rotate_vec2(v: Vec2, radians: f32) -> Vec2 {
    let sin = radians.sin();
    let cos = radians.cos();
    let x = v.x * cos - v.y * sin;
    let y = v.x * sin + v.y * cos;
    Vec2::new(x, y)
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * clamp_01(t)
}
