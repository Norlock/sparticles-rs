use macroquad::prelude::draw_circle;
use macroquad::prelude::Color;

use crate::position::Position;
use crate::transform::Transform;

#[derive(Debug)]
pub struct Particle {
    pub inqueue: bool,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub diameter: f32,
    pub color: Color,
    pub decay: f32,
    pub vx_energy: u16,
    pub vy_energy: u16,
    pub frame: u16,
}

pub struct ParticleAttributes {
    pub diameter: f32,
    pub color: Color,
    pub decay: f32,
}

// TODO add factory that returns mesh based on particle
impl Particle {
    pub fn new(x: f32, y: f32, attributes: &ParticleAttributes) -> Self {
        Self {
            x,
            y,
            vx: 0.1,
            vy: 0.,
            decay: attributes.decay,
            color: attributes.color.clone(),
            diameter: attributes.diameter,
            frame: 0,
            vx_energy: 0,
            vy_energy: 0,
            inqueue: false,
        }
    }

    pub fn draw(&self, grid_position: &Position) {
        draw_circle(
            self.x + grid_position.x,
            self.y + grid_position.y,
            self.diameter,
            self.color,
        );
    }

    pub fn transform(&mut self, transform: Transform) {
        self.vx = transform.vx;
        self.vy = transform.vy;
        self.x = self.x + self.vx;
        self.y = self.y + self.vy;
    }
}
