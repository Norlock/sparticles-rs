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
    pub mass: f32,
    /// number between 1 and 0. (percentage of bounciness).
    pub elasticity_fraction: f32,
    /// number in Newton (m * g).
    pub friction: f32,
    pub animation: Rc<Animate>,
    pub frame: u32,
    pub last_frame: u32,
}

pub enum InitFrame {
    Zero,
    Random,
}

pub struct ParticleAttributes {
    /// number between 1 and 0. (percentage of bounciness).
    pub elasticity_fraction: f32,
    /// number in Newton (m * g).
    pub friction: f32,

    pub color: Color,
    pub mass: f32,
    pub diameter: f32,
    pub animation: Rc<Animate>,
    pub last_frame: u32,
    pub init_frame: InitFrame,
}

impl Particle {
    pub fn new(x: f32, y: f32, attributes: &ParticleAttributes) -> Self {
        let frame = match attributes.init_frame {
            InitFrame::Zero => 0,
            InitFrame::Random => rand::gen_range(0, attributes.last_frame),
        };

        Self {
            x,
            y,
            vx: 1.5,
            vy: 1.,
            friction: attributes.friction,
            color: attributes.color.clone(),
            radius: attributes.diameter / 2.,
            diameter: attributes.diameter,
            elasticity_fraction: attributes.elasticity_fraction,
            mass: attributes.mass,
            queue_frame: UINT32_MAX,
            animation: Rc::clone(&attributes.animation),
            last_frame: attributes.last_frame,
            frame,
        }
    }

    pub fn handle_possible_collision(
        &self,
        other: &mut Particle,
        transform: &mut Transform,
        new_x: f32,
        new_y: f32,
        end_self_x: f32,
        end_self_y: f32,
    ) {
        let end_other_x = other.x + other.diameter;
        let end_other_y = other.y + other.diameter;

        let inside_x = other.x <= new_x && new_x <= end_other_x
            || other.x <= end_self_x && end_self_x <= end_other_x;
        let inside_y = other.y <= new_y && new_y <= end_other_y
            || other.y <= end_self_y && end_self_y <= end_other_y;

        // No collision
        if !inside_x || !inside_y {
            return;
        }

        if self.mass == other.mass {
            let tmp = transform.vx;
            transform.vx = other.vx * self.elasticity_fraction;
            other.vx = tmp * other.elasticity_fraction;

            let tmp = transform.vy;
            transform.vy = other.vy * self.elasticity_fraction;
            other.vy = tmp * other.elasticity_fraction;
            return;
        }

        let total_weight = self.mass + other.mass;

        let transform_vx = ((self.mass - other.mass) / total_weight * transform.vx)
            + (2. * other.mass / total_weight * other.vx);
        let other_vx = (2. * self.mass / total_weight * transform.vx)
            + ((other.mass - self.mass) / total_weight * other.vx);

        transform.vx = transform_vx * self.elasticity_fraction;
        other.vx = other_vx * other.elasticity_fraction;

        let transform_vy = ((self.mass - other.mass) / total_weight * transform.vy)
            + (2. * other.mass / total_weight * other.vy);
        let other_vy = (2. * self.mass / total_weight * transform.vy)
            + ((other.mass - self.mass) / total_weight * other.vy);

        transform.vy = transform_vy * self.elasticity_fraction;
        other.vx = other_vy * other.elasticity_fraction;
    }

    pub fn update(
        &mut self,
        grid_position: &Position,
        transform: Transform,
        max_width: f32,
        max_height: f32,
    ) {
        self.animate();
        self.transform(transform, max_width, max_height);
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

    fn transform(&mut self, transform: Transform, max_width: f32, max_height: f32) {
        self.vx = transform.vx;
        self.vy = transform.vy;
        self.x += transform.vx;
        self.y += transform.vy;

        if self.x < 0. {
            self.x = 0.;
        } else if max_width <= self.x + self.diameter {
            self.x = max_width - 1. - self.diameter;
        }

        if self.y < 0. {
            self.y = 0.;
        } else if max_height <= self.y + self.diameter {
            self.y = max_height - 1. - self.diameter;
        }
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
