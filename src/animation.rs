use macroquad::prelude::Color;

pub type Animate = fn(data: &mut AnimationData, frame: u32);

pub struct AnimationData {
    pub color: Color,
    pub diameter: f32,
}

// Animation design
struct Animation {
    pub last_frame: u32,
    pub animate: Animate,
}

impl Animation {
    pub fn new(last_frame: u32, animate: Animate) -> Self {
        Self {
            animate,
            last_frame,
        }
    }
}
