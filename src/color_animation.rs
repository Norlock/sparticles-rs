use std::time::Duration;
use std::time::Instant;

use macroquad::prelude::Color;

use crate::animation::Animatee;
use crate::animation::AnimationData;

#[derive(Clone, Debug)]
pub struct ColorAnimation {
    pub color1: Color,
    pub color2: Color,
    pub from_ms: u128,
    pub until_ms: u128,
}

impl Animatee for ColorAnimation {
    fn animate(&self, data: &mut AnimationData, lifetime: &Instant) {
        let elapsed_ms = lifetime.elapsed().as_millis();

        if elapsed_ms < self.from_ms || self.until_ms < elapsed_ms {
            return;
        }

        let delta_current = elapsed_ms - self.from_ms;
        let delta_max = self.until_ms - elapsed_ms;

        // calculate percent
        let percent = delta_current as f32 / delta_max as f32;
        data.color.r = self.color1.r + percent * (self.color2.r - self.color1.r);
        data.color.g = self.color1.g + percent * (self.color2.g - self.color1.g);
        data.color.b = self.color1.b + percent * (self.color2.b - self.color1.b);
    }
}
