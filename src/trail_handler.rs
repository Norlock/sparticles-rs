use crate::trail_animation::TrailAnimation;
use crate::trail_animation::TrailData;

#[derive(Debug, Clone)]
pub struct TrailHandler {
    pub duration_ms: u32,
    pub trail_animations: Vec<TrailAnimation>,
}

impl TrailHandler {
    pub fn animate(&mut self, data: &mut TrailData, elapsed_ms: u128) {
        let cycle_ms = elapsed_ms as u32 % self.duration_ms;
        for animation in self.trail_animations.iter_mut() {
            animation.animate(data, cycle_ms);
        }
    }
}
