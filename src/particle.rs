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

    fn set_vx_force(&self, transform: &mut Transform, other: &mut Particle) {
        if self.weight == other.weight {
            let tmp = transform.vx();
            transform.set_new_vx(other.vx);
            other.vx = tmp;
            return;
        }

        let total_weight = self.weight + other.weight;
        let transform_vx = ((self.weight - other.weight) / total_weight * transform.vx())
            + (2. * other.weight / total_weight * other.vx);
        let other_vx = (2. * self.weight / total_weight * transform.vx())
            + ((other.weight - self.weight) / total_weight * other.vx);

        let elasticity_fraction = (self.elasticity_fraction + other.elasticity_fraction) / 2.;
        transform.set_new_vx(transform_vx * elasticity_fraction);
        other.vx = other_vx * elasticity_fraction;
    }

    fn set_vy_force(&self, transform: &mut Transform, other: &mut Particle) {
        if self.weight == other.weight {
            let tmp = transform.vy();
            transform.set_new_vy(other.vy);
            other.vy = tmp;
            return;
        }

        let total_weight = self.weight + other.weight;
        let transform_vy = ((self.weight - other.weight) / total_weight * transform.vy())
            + (2. * other.weight / total_weight * other.vy);
        let other_vy = (2. * self.weight / total_weight * transform.vy())
            + ((other.weight - self.weight) / total_weight * other.vy);

        let elasticity_fraction = (self.elasticity_fraction + other.elasticity_fraction) / 2.;
        transform.set_new_vy(transform_vy * elasticity_fraction);
        other.vx = other_vy * elasticity_fraction;
    }

    pub fn handle_collision(&self, other: &mut Particle, transform: &mut Transform) {
        let inside_x =
            other.x <= transform.new_x() && transform.new_x() <= other.x + other.diameter;
        let inside_y =
            other.y <= transform.new_y() && transform.new_y() <= other.y + other.diameter;

        if inside_x && inside_y {
            // TODO incorporate weight.
            self.set_vx_force(transform, other);
            self.set_vy_force(transform, other);

            transform.set_new_vx(transform.vx());
            transform.set_new_vy(transform.vy());
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
