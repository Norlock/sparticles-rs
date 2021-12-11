use macroquad::prelude::Color;

pub type Animate = fn(data: &mut AnimationData, frame: u32);

pub struct AnimationData {
    pub color: Color,
    pub diameter: f32,
}
