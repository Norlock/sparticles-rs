use crate::animation::Animate;
use crate::collision::CollisionData;
use macroquad::miniquad::gl::UINT32_MAX;
use macroquad::prelude::draw_circle;
use macroquad::prelude::rand;
use macroquad::prelude::Color;
use std::rc::Rc;

use crate::position::Position;

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
    pub mass: f32,
    /// number between 1 and 0. (percentage of bounciness).
    pub elasticity_fraction: f32,
    /// number in Newton (m * g).
    pub friction: f32,
    //pub animation: Rc<Animate>,
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
            vx: 0.,
            vy: 0.,
            friction: attributes.friction,
            color: attributes.color.clone(),
            radius: attributes.diameter / 2.,
            diameter: attributes.diameter,
            elasticity_fraction: attributes.elasticity_fraction,
            mass: attributes.mass,
            queue_frame: UINT32_MAX,
            last_frame: attributes.last_frame,
            frame,
        }
    }

    fn move_if_overlaps(&mut self, other: &mut Particle) {
        let new_x = self.x + self.vx;
        let new_y = self.y + self.vy;
        let other_new_x = other.x + other.vx;
        let other_new_y = other.y + other.vy;
        let end_other_new_x = other_new_x + other.diameter;
        let end_other_new_y = other_new_y + other.diameter;

        let x_difference = new_x - other_new_x;
        let y_difference = new_y - other_new_y;

        let collision_self_placed_right = 0. < x_difference && x_difference < other.diameter;
        let collision_self_placed_left = -self.diameter < x_difference && x_difference < 0.;
        let collision_self_placed_bottom = 0. < y_difference && y_difference < other.diameter;
        let collision_self_placed_top = -self.diameter < y_difference && y_difference < 0.;

        let x_collision = collision_self_placed_left || collision_self_placed_right;
        let y_collision = collision_self_placed_top || collision_self_placed_bottom;

        if !x_collision || !y_collision {
            return;
        }

        // The higher the difference the lower the overlap is.
        // In case of overlap, you move back the least overlapping part.
        // e.g. both particles diameter is 5.
        // p1,x == 9, p2,x == 5. Remainder == 9 - 5. (4)
        // p1,y == 8, p2,y == 5. Remainder == 8 - 5. (3)
        // if x1 moves to the right with 1x its out of bound instead of y1 moving with 2y.
        let move_back_horizontally = y_difference.abs() < x_difference.abs();

        if move_back_horizontally {
            if collision_self_placed_right {
                if 0. <= self.vx {
                    self.x = end_other_new_x - self.vx + 0.01;
                } else {
                    self.x = end_other_new_x + self.vx + 0.01;
                }
            } else {
                if 0. <= other.vx {
                    self.x = other_new_x - self.diameter - other.vx - 0.01;
                } else {
                    self.x = other_new_x - self.diameter + other.vx - 0.01;
                }
            }
        } else {
            if collision_self_placed_bottom {
                if 0. <= self.vy {
                    self.y = end_other_new_y - self.vy + 0.01;
                } else {
                    self.y = end_other_new_y + self.vy + 0.01;
                }
            } else {
                if 0. <= other.vy {
                    self.y = other_new_y - self.diameter - other.vy - 0.01;
                } else {
                    self.y = other_new_y - self.diameter + other.vy - 0.01;
                }
            }
        }
    }

    pub fn handle_possible_collision(
        &mut self,
        other: &mut Particle,
        data: &mut CollisionData,
    ) -> bool {
        let CollisionData {
            new_x,
            new_y,
            end_new_x,
            end_new_y,
        } = *data;

        let other_new_x = other.x + other.vx;
        let other_new_y = other.y + other.vy;
        let end_other_new_x = other_new_x + other.diameter;
        let end_other_new_y = other_new_y + other.diameter;

        let inside_x = other_new_x <= new_x && new_x <= end_other_new_x
            || other_new_x <= end_new_x && end_new_x <= end_other_new_x;
        let inside_y = other_new_y <= new_y && new_y <= end_other_new_y
            || other_new_y <= end_new_y && end_new_y <= end_other_new_y;

        if !inside_x || !inside_y {
            // No collision
            return false;
        }

        if self.mass == other.mass {
            let tmp = self.vx;
            self.vx = other.vx * self.elasticity_fraction;
            other.vx = tmp * other.elasticity_fraction;

            let tmp = self.vy;
            self.vy = other.vy * self.elasticity_fraction;
            other.vy = tmp * other.elasticity_fraction;
            self.move_if_overlaps(other);
            return true;
        }

        let total_weight = self.mass + other.mass;

        let transform_vx = ((self.mass - other.mass) / total_weight * self.vx)
            + (2. * other.mass / total_weight * other.vx);
        let other_vx = (2. * self.mass / total_weight * self.vx)
            + ((other.mass - self.mass) / total_weight * other.vx);

        self.vx = transform_vx * self.elasticity_fraction;
        other.vx = other_vx * other.elasticity_fraction;

        let transform_vy = ((self.mass - other.mass) / total_weight * self.vy)
            + (2. * other.mass / total_weight * other.vy);
        let other_vy = (2. * self.mass / total_weight * self.vy)
            + ((other.mass - self.mass) / total_weight * other.vy);

        self.vy = transform_vy * self.elasticity_fraction;
        other.vx = other_vy * other.elasticity_fraction;

        self.move_if_overlaps(other);
        return true;
    }

    pub fn update(&mut self, grid_position: &Position, max_width: f32, max_height: f32) {
        self.animate();
        self.transform(max_width, max_height);
        self.draw(grid_position);
    }

    fn animate(&mut self) {
        //let mut data = AnimationData {
        //color: self.color,
        //diameter: self.diameter,
        //};

        //let animation = &self.animation;
        //animation(&mut data, self.frame);

        //self.color = data.color;
        //self.diameter = self.diameter;
        //self.radius = self.diameter / 2.;

        //if self.frame == self.last_frame {
        //self.frame = 0;
        //} else {
        //self.frame += 1;
        //}
    }

    fn draw(&self, grid_position: &Position) {
        draw_circle(
            self.x + grid_position.x,
            self.y + grid_position.y,
            self.radius,
            self.color,
        );
    }

    fn transform(&mut self, max_width: f32, max_height: f32) {
        self.x += self.vx;
        self.y += self.vy;

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
