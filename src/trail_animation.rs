use crate::animation::Animate;
use macroquad::prelude::*;

use crate::animation::AnimationData;

pub struct TrailAnimation {
    /// When to update trail, (lower == ^ precision / higher == ^ performance).
    /// 32ms is a good starting point.
    new_line_point_ms: u128,

    trail_length: usize,
    /// number between 0..1 (e.g. 0.1)
    opacity_loss_per_point: f32,

    diameter_fraction: f32,
    from_ms: u128,
    until_ms: u128,
}

#[derive(Debug, Clone, Copy)]
pub struct TrailPoint {
    pub x: f32,
    pub y: f32,
    pub color: Color,
    pub iteration: u16,
}

pub struct TrailOptions {
    /// When to update trail, (lower == ^ precision / higher == ^ performance).
    pub new_line_point_ms: u128,
    /// number between 0..1
    pub opacity_loss_per_point: f32,
    /// Diameter fraction from 0..1
    pub diameter_fraction: f32,

    pub from_ms: u128,
    pub until_ms: u128,
}

impl TrailAnimation {
    pub fn new(options: TrailOptions) -> Self {
        let TrailOptions {
            new_line_point_ms,
            opacity_loss_per_point,
            diameter_fraction,
            from_ms,
            until_ms,
        } = options;

        let trail_length = (1. / opacity_loss_per_point).ceil() as usize;

        Self {
            from_ms,
            until_ms,
            new_line_point_ms,
            trail_length,
            opacity_loss_per_point,
            diameter_fraction,
        }
    }

    fn draw(&self, data: &mut AnimationData, new_iteration: u16) {
        let diameter = data.radius * 2. * self.diameter_fraction;

        data.trail_abs.iter_mut().reduce(|from, to| {
            let point_difference = new_iteration - from.iteration;
            let alpha_loss = self.opacity_loss_per_point * point_difference as f32;
            from.color.a = 1. - alpha_loss;

            if alpha_loss < 1. && from.iteration + 1 == to.iteration {
                draw_line(from.x, from.y, to.x, to.y, diameter, from.color);
            }
            to
        });
    }

    fn clean_up_trail(&self, data: &mut AnimationData) {
        data.trail_abs
            .retain(|trail_point| 0. < trail_point.color.a);
    }

    fn update_last(&self, data: &mut AnimationData) {
        let last = &mut data.trail_abs.last_mut().unwrap();
        // Iteration stays unchanged.
        last.x = data.point_abs.0;
        last.y = data.point_abs.1;
        last.color = data.color;
    }
}

impl Animate for TrailAnimation {
    fn animate(&self, data: &mut AnimationData<'_>, elapsed_ms: u128) {
        let new_iteration = (elapsed_ms / self.new_line_point_ms) as u16;
        if elapsed_ms < self.from_ms || self.until_ms <= elapsed_ms {
            self.draw(data, new_iteration);
            self.clean_up_trail(data);
            return;
        }

        let create_trail_point = || TrailPoint {
            color: data.color,
            iteration: new_iteration,
            x: data.point_abs.0,
            y: data.point_abs.1,
        };

        if data.trail_abs.is_empty() {
            data.trail_abs.push(create_trail_point());
            return;
        }

        let last_point = data.trail_abs.last().unwrap();
        if last_point.iteration < new_iteration {
            if self.trail_length == data.trail_abs.len() {
                data.trail_abs.drain(0..1);
            }
            data.trail_abs.push(create_trail_point());
        } else {
            self.update_last(data);
        }

        self.draw(data, new_iteration);
    }
}
