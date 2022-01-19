use macroquad::prelude::Color;
use std::fmt;
use std::fmt::Debug;
use std::time::Instant;

pub type Animate = fn(data: &mut AnimationData);

pub trait Animatee {
    fn animate(&self, data: &mut AnimationData, lifetime: &Instant);
}

impl Debug for dyn Animatee {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Animate")
    }
}

pub struct AnimationData {
    pub color: Color,
    pub diameter: f32,
    pub vx: f32,
    pub vy: f32,
    pub raw_frame_counter: u32,
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
