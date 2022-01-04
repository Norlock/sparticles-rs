use macroquad::prelude::Color;
use std::fmt;

pub type Animate = fn(data: &mut AnimationData);

pub struct AnimationData {
    pub color: Color,
    pub diameter: f32,
    pub vx: f32,
    pub vy: f32,
}

#[derive(Clone)]
pub enum Animation {
    TimeBased {
        start: u32,
        until: u32,
        animate: Animate,
    },
    Allways(Animate),
}

impl fmt::Debug for Animation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Animation").finish()
    }
}
