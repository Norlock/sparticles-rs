use macroquad::prelude::Color;

use crate::animation::animation::Animate;
use crate::animation::animation::AnimationData;
use crate::animation::animation::AnimationTime;

#[derive(Clone, Debug)]
pub struct ColorAnimation {
    pub color1: Color,
    pub color2: Color,
    pub from_ms: u32,
    pub until_ms: u32,
}

impl Animate for ColorAnimation {
    fn animate(&self, data: &mut AnimationData, time: &AnimationTime) {
        if time.cycle_ms < self.from_ms || self.until_ms <= time.cycle_ms {
            return;
        }

        let delta_current = time.cycle_ms - self.from_ms;
        let delta_max = self.until_ms - self.from_ms;

        // calculate percent from 0..1
        let fraction = delta_current as f32 / delta_max as f32;
        data.color.r = self.color1.r + fraction * (self.color2.r - self.color1.r);
        data.color.g = self.color1.g + fraction * (self.color2.g - self.color1.g);
        data.color.b = self.color1.b + fraction * (self.color2.b - self.color1.b);
        data.color.a = self.color1.a + fraction * (self.color2.a - self.color1.a);
    }
}
