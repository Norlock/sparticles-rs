use crate::animation::Animate;
use crate::AnimationData;
use macroquad::miniquad::gl::UINT32_MAX;
use macroquad::prelude::draw_circle;
use macroquad::prelude::rand;
use macroquad::prelude::Color;
use std::fmt;
use std::rc::Rc;

use crate::position::Position;
use crate::transform::Transform;

pub struct Particle {
    pub queue_frame: u32,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub radius: f32,
    pub diameter: f32,
    pub color: Color,
    pub weight: f32,
    /// number between 1 and 0. (percentage of bounciness).
    pub elasticity_fraction: f32,
    /// number between 1 and 0. (percentage of loss).
    pub decay_fraction: f32,
    pub animation: Rc<Animate>,
    pub frame: u32,
    pub last_frame: u32,
}

pub enum InitFrame {
    Zero,
    Random,
}

pub struct ParticleAttributes {
    pub color: Color,
    /// number between 1 and 0. (percentage of bounciness).
    pub elasticity_fraction: f32,
    /// number between 1 and 0. (percentage of loss).
    pub decay_fraction: f32,
    pub weight: f32,
    pub diameter: f32,
    pub animation: Rc<Animate>,
    pub last_frame: u32,
    pub init_frame: InitFrame,
}

fn set_vx_force(transform: &mut Transform, other: &mut Particle) {
    // swap energies
    let tmp_vx = transform.vx();
    transform.set_new_vx(other.vx);
    other.vx = tmp_vx;
    // TODO swap velocities is only correct if there is no difference in weight.
    // in future calculate energy

    //if 0. < transform.vx() && 0. < other.vx || transform.vx() < 0. && other.vx < 0. {
    //let tmp_vx = transform.vx();
    //transform.set_new_vx(other.vx);
    //other.vx = tmp_vx;
    //} else {
    //transform.set_new_vx(transform.vx() + other.vx);
    //}
}

fn set_vy_force(transform: &mut Transform, other: &mut Particle) {
    // swap energies
    let tmp_vy = transform.vy();
    transform.set_new_vy(other.vy);
    other.vy = tmp_vy;
    // TODO swap velocities is only correct if there is no difference in weight.
    // in future calculate energy

    //if 0. < transform.vy() && 0. < other.vy {
    //transform.set_new_vx(transform.vy().max(other.vy));
    //} else if transform.vx() < 0. && other.vy < 0. {
    //transform.set_new_vx(transform.vy().min(other.vy));
    //} else {
    //transform.set_new_vx(transform.vy() + other.vy);
    //}
}

// TODO add factory that returns mesh based on particle
impl Particle {
    pub fn new(x: f32, y: f32, attributes: &ParticleAttributes) -> Self {
        let mut frame = 0;

        match attributes.init_frame {
            InitFrame::Zero => (),
            InitFrame::Random => {
                frame = rand::gen_range(0, attributes.last_frame);
            }
        }

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
            weight: attributes.weight,
            queue_frame: UINT32_MAX,
            animation: Rc::clone(&attributes.animation),
            last_frame: attributes.last_frame,
            frame,
        }
    }

    pub fn handle_collision(&self, other: &mut Particle, transform: &mut Transform) {
        let inside_x =
            other.x <= transform.new_x() && transform.new_x() <= other.x + other.diameter;
        let inside_y =
            other.y <= transform.new_y() && transform.new_y() <= other.y + other.diameter;

        if inside_x && inside_y {
            //let
            // TODO incorporate weight.
            set_vx_force(transform, other);
            set_vy_force(transform, other);

            transform.set_new_vx(transform.vx() * self.elasticity_fraction);
            transform.set_new_vy(transform.vy() * self.elasticity_fraction);
        }
    }

    pub fn update(&mut self, grid_position: &Position, transform: Transform) {
        self.animate();
        self.transform(transform);
        self.draw(grid_position);
    }

    fn animate(&mut self) {
        let mut data = AnimationData {
            color: self.color,
            diameter: self.diameter,
        };

        let animation = &self.animation;
        animation(&mut data, self.frame);

        self.color = data.color;
        self.diameter = self.diameter;
        self.radius = self.diameter / 2.;

        if self.frame == self.last_frame {
            self.frame = 0;
        } else {
            self.frame += 1;
        }
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

impl fmt::Debug for Particle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Particle")
            //.field("max_frame", &self.last_frame)
            //.field("current_frame", &self.frame)
            .finish()
    }
}
