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
