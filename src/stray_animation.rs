use crate::animation::Animate;
use crate::animation::AnimationData;
use macroquad::prelude::rand;

pub struct StrayAnimation {
    from_ms: u128,
    until_ms: u128,
    strayness_radians: f32,
}

impl StrayAnimation {
    pub fn new(from_ms: u128, until_ms: u128, strayness_degrees: f32) -> Self {
        Self {
            from_ms,
            until_ms,
            strayness_radians: strayness_degrees.to_radians(),
        }
    }
}

impl Animate for StrayAnimation {
    fn animate(&self, data: &mut AnimationData, animation_cycle_ms: u128) {
        if animation_cycle_ms < self.from_ms || self.until_ms <= animation_cycle_ms {
            return;
        }

        let stray = rand::gen_range(-self.strayness_radians, self.strayness_radians);
        data.vx = (data.vx * stray.cos()) - (data.vy * stray.sin());
        data.vy = (data.vx * stray.sin()) + (data.vy * stray.cos());
    }
}
