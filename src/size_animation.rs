use crate::animation::Animate;
use crate::animation::AnimationData;

#[derive(Clone, Debug)]
pub struct SizeAnimation {
    pub start_radius: f32,
    pub end_radius: f32,
    pub from_ms: u128,
    pub until_ms: u128,
}

impl Animate for SizeAnimation {
    fn animate(&self, data: &mut AnimationData, anim_cycle_ms: u128) {
        if anim_cycle_ms < self.from_ms || self.until_ms <= anim_cycle_ms {
            return;
        }

        let delta_current = anim_cycle_ms - self.from_ms;
        let delta_max = self.until_ms - self.from_ms;

        // calculate percent
        let fraction = delta_current as f32 / delta_max as f32;
        data.radius = self.start_radius + fraction * (self.end_radius - self.start_radius);
    }
}
