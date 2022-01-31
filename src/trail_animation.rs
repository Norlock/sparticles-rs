use crate::animation::Animate;
use macroquad::prelude::*;

use crate::animation::AnimationData;

pub struct TrailAnimation {
    /// How many iterations of update_ms.
    trail_length: usize,
    /// When to update trail, (lower == ^ precision / higher == ^ performance).
    iteration_length_ms: u128,
    /// Opacity fraction from 0..1
    opacity_from: f32,
    opacity_to: f32,
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
    /// How many iterations of update_ms.
    pub trail_length: usize,
    /// When to update trail, (lower == ^ precision / higher == ^ performance).
    pub iteration_length_ms: u128,
    /// Opacity fraction from 0..1
    pub opacity_from: f32,
    /// Opacity fraction from 0..1
    pub opacity_to: f32,
    /// Diameter fraction from 0..1
    pub diameter_fraction: f32,

    pub from_ms: u128,
    pub until_ms: u128,
}

impl TrailAnimation {
    pub fn new(options: TrailOptions) -> Self {
        let TrailOptions {
            trail_length,
            iteration_length_ms,
            opacity_from,
            opacity_to,
            diameter_fraction,
            from_ms,
            until_ms,
        } = options;

        Self {
            from_ms,
            until_ms,
            iteration_length_ms,
            trail_length,
            opacity_from,
            opacity_to,
            diameter_fraction,
        }
    }

    fn draw(&self, data: &mut AnimationData) {
        let diameter = data.radius * 2. * self.diameter_fraction;
        let until = data.trail_abs.len();
        let mut i = 1;

        let opacity_delta = self.opacity_to - self.opacity_from;
        data.trail_abs.iter_mut().reduce(|from, to| {
            let fraction = i as f32 / until as f32;
            from.color.a = self.opacity_from + fraction * opacity_delta;
            draw_line(from.x, from.y, to.x, to.y, diameter, from.color);
            i += 1;
            to
        });
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
        if elapsed_ms < self.from_ms || self.until_ms <= elapsed_ms {
            return;
        }

        let new_iteration = (elapsed_ms / self.iteration_length_ms) as u16;

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
        self.draw(data);
    }
}
