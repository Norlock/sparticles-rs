use macroquad::prelude::Color;
use std::fmt::Debug;

pub trait Animate {
    fn animate(&self, data: &mut AnimationData, animation_cycle_ms: u128);
}

impl Debug for dyn Animate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Animate")
    }
}

pub struct AnimationData {
    pub color: Color,
    pub radius: f32,
    pub vx: f32,
    pub vy: f32,
}
