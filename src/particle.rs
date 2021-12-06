use macroquad::prelude::draw_circle;
use macroquad::prelude::Color;

#[derive(Debug)]
pub struct Particle {
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
    pub cell_x_index: u16,
    pub cell_y_index: u16,
}

pub struct ParticleAttributes {
    pub diameter: f32,
    pub color: Color,
    pub decay: f32,
}

// TODO add factory that returns mesh based on particle
impl Particle {
    pub fn new(
        x: f32,
        y: f32,
        cell_x_index: u16,
        cell_y_index: u16,
        attributes: &ParticleAttributes,
    ) -> Self {
        Self {
            x,
            y,
            vx: 1.,
            vy: 0.,
            decay: attributes.decay,
            color: attributes.color.clone(),
            diameter: attributes.diameter,
            frame: 0,
            vx_energy: 0,
            vy_energy: 0,
            cell_x_index,
            cell_y_index,
        }
    }

    pub fn draw(&self) {
        draw_circle(self.x, self.y, self.diameter, self.color);
    }
}
