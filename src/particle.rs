use macroquad::miniquad::gl::UINT32_MAX;
use macroquad::prelude::draw_circle;
use macroquad::prelude::Color;

use crate::position::Position;
use crate::transform::Transform;

#[derive(Debug)]
pub struct Particle {
    pub queue_frame: u32,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub radius: f32,
    pub diameter: f32,
    pub color: Color,
    pub frame: u16,
    /// number between 1 and 0. (percentage of bounciness).
    pub elasticity_fraction: f32,
    /// number between 1 and 0. (percentage of loss).
    pub decay_fraction: f32,
}

pub struct ParticleAttributes {
    pub color: Color,
    /// number between 1 and 0. (percentage of bounciness).
    pub elasticity_fraction: f32,
    /// number between 1 and 0. (percentage of loss).
    pub decay_fraction: f32,
    pub weight: f32,
    pub diameter: f32,
}

fn set_vx_force(transform: &mut Transform, other: &Particle) {
    if 0. < transform.vx() && 0. < other.vx {
        transform.set_new_vx(transform.vx().max(other.vx));
    } else if transform.vx() < 0. && other.vx < 0. {
        transform.set_new_vx(transform.vx().min(other.vx));
    } else {
        transform.set_new_vx(transform.vx() + other.vx);
    }
}

fn set_vy_force(transform: &mut Transform, other: &Particle) {
    if 0. < transform.vy() && 0. < other.vy {
        transform.set_new_vx(transform.vy().max(other.vy));
    } else if transform.vx() < 0. && other.vy < 0. {
        transform.set_new_vx(transform.vy().min(other.vy));
    } else {
        transform.set_new_vx(transform.vy() + other.vy);
    }
}

// TODO add factory that returns mesh based on particle
impl Particle {
    pub fn new(x: f32, y: f32, attributes: &ParticleAttributes) -> Self {
        Self {
            x,
            y,
            vx: 1.5,
            vy: 1.,
            decay_fraction: attributes.decay_fraction,
            color: attributes.color.clone(),
            radius: attributes.diameter / 2.,
            diameter: attributes.diameter,
            elasticity_fraction: attributes.elasticity_fraction,
            frame: 0,
            queue_frame: UINT32_MAX,
        }
    }

    pub fn handle_collision(&self, other: &Particle, transform: &mut Transform) {
        let inside_x =
            other.x <= transform.new_x() && transform.new_x() <= other.x + other.diameter;
        let inside_y =
            other.y <= transform.new_y() && transform.new_y() <= other.y + other.diameter;

        if inside_x && inside_y {
            //let
            // TODO incorporate weight.
            set_vx_force(transform, other);
            set_vy_force(transform, other);
            transform.set_new_vx(transform.vx() * (-1. * self.elasticity_fraction));
            transform.set_new_vy(transform.vy() * (-1. * self.elasticity_fraction));
        }
    }

    pub fn update(&mut self, grid_position: &Position, transform: Transform) {
        self.transform(transform);
        self.draw(grid_position);
    }

    fn draw(&self, grid_position: &Position) {
        draw_circle(
            self.x + grid_position.x,
            self.y + grid_position.y,
            self.radius,
            self.color,
        );
    }

    fn transform(&mut self, transform: Transform) {
        self.vx = transform.vx();
        self.vy = transform.vy();
        self.x = transform.new_x();
        self.y = transform.new_y();
    }
}
