use macroquad::prelude::*;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct TrailAnimation {
    /// When to update trail, (lower is more precision / higher is better performance).
    /// 32ms is a good starting point.
    update_ms: u32,

    /// number between 0..1 (e.g. 0.1)
    opacity_loss_per_update: f32,

    diameter_fraction: f32,
    from_ms: u32,
    until_ms: u32,
    trail: VecDeque<TrailPoint>,
    iteration: u32,
}

#[derive(Debug, Clone, Copy)]
struct TrailPoint {
    x: f32,
    y: f32,
    color: Color,
    diameter: f32,
    line_end: bool,
}

pub struct TrailOptions {
    /// When to update trail, (lower == ^ precision / higher == ^ performance).
    /// Min is 16.
    pub update_ms: u32,
    /// number between 0..1
    pub opacity_loss_per_update: f32,
    /// Diameter fraction from 0..1
    pub diameter_fraction: f32,

    pub from_ms: u32,
    pub until_ms: u32,
}

pub struct TrailData {
    pub x_abs: f32,
    pub y_abs: f32,
    pub color: Color,
    pub radius: f32,
}

impl TrailAnimation {
    pub fn new(options: TrailOptions) -> Self {
        let TrailOptions {
            mut update_ms,
            opacity_loss_per_update,
            diameter_fraction,
            from_ms,
            until_ms,
        } = options;

        update_ms = update_ms.max(16);

        Self {
            from_ms,
            until_ms,
            update_ms,
            opacity_loss_per_update,
            diameter_fraction,
            iteration: 0,
            trail: VecDeque::new(),
        }
    }

    pub fn animate(&mut self, data: &TrailData, cycle_ms: u32) {
        let new_iteration = cycle_ms / self.update_ms;

        let is_in_cycle = self.from_ms <= cycle_ms && cycle_ms < self.until_ms;
        let is_new_iteration = self.iteration != new_iteration;

        self.trail.iter_mut().reduce(|from, to| {
            if is_new_iteration {
                from.color.a -= self.opacity_loss_per_update;
            }

            if !from.line_end {
                draw_line(from.x, from.y, to.x, to.y, to.diameter, to.color);
            }

            to
        });

        if is_new_iteration {
            self.iteration = new_iteration;
            self.trail.retain(|point| point.color.a.is_sign_positive());
        }

        if is_in_cycle {
            let diameter = data.radius * 2. * self.diameter_fraction;
            let new_x = data.x_abs;
            let new_y = data.y_abs;

            let create_new_point = || TrailPoint {
                color: data.color,
                x: new_x,
                y: new_y,
                line_end: false,
                diameter,
            };

            if self.trail.is_empty() || is_new_iteration {
                self.trail.push_back(create_new_point());
            } else {
                let last_index = self.trail.len() - 1;
                let lp = &mut self.trail[last_index];
                lp.x = new_x;
                lp.y = new_y;
                lp.color = data.color;
                lp.diameter = diameter;
            }
        } else {
            self.trail
                .iter_mut()
                .last()
                .map(|last_point| last_point.line_end = true);
        }
    }
}
