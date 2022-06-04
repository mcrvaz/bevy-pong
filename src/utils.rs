use bevy::prelude::*;

pub fn v2_to_v3(v2: Vec2) -> Vec3 {
    Vec3::new(v2.x, v2.y, 0.0)
}
