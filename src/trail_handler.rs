use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct TrailHandler {
    trail: Vec<ColorPoint>,
    /// How many iterations of update_ms.
    trail_length: usize,
    /// When to update trail, (lower == ^ precision / higher == ^ performance).
    update_ms: u128,
    /// Opacity fraction from 0..1
    opacity_from: f32,
    opacity_to: f32,
    diameter_fraction: f32,
    iteration: i16,
}

#[derive(Debug, Clone, Copy)]
pub struct ColorPoint {
    pub x: f32,
    pub y: f32,
    pub color: Color,
}

pub struct TrailOptions {
    /// How many iterations of update_ms.
    pub trail_length: usize,
    /// When to update trail, (lower == ^ precision / higher == ^ performance).
    pub update_ms: u128,
    /// Opacity fraction from 0..1
    pub opacity_from: f32,
    /// Opacity fraction from 0..1
    pub opacity_to: f32,
    /// Diameter fraction from 0..1
    pub diameter_fraction: f32,
}

impl TrailHandler {
    pub fn new(options: TrailOptions) -> Self {
        let TrailOptions {
            trail_length,
            update_ms,
            opacity_from,
            opacity_to,
            diameter_fraction,
        } = options;

        Self {
            update_ms,
            iteration: -1,
            trail_length,
            opacity_from,
            opacity_to,
            diameter_fraction,
            trail: Vec::new(),
        }
    }

    fn draw(&mut self, radius: f32) {
        let diameter = radius * 2. * self.diameter_fraction;
        let until = self.trail.len();
        let mut i = 1;

        let opacity_delta = self.opacity_to - self.opacity_from;
        self.trail.iter_mut().reduce(|from, to| {
            let fraction = i as f32 / until as f32;
            from.color.a = self.opacity_from + fraction * opacity_delta;
            draw_line(from.x, from.y, to.x, to.y, diameter, from.color);
            i += 1;
            to
        });
    }

    fn swap_last(&mut self, current: ColorPoint) {
        let index = self.trail.len() - 1;
        self.trail[index] = current;
    }

    pub fn update(&mut self, current: ColorPoint, radius: f32, elapsed_ms: u128) {
        let new_iteration = elapsed_ms / self.update_ms;

        if self.iteration < new_iteration as i16 {
            if self.trail_length == self.trail.len() {
                self.trail.drain(0..1);
            }

            self.iteration = new_iteration as i16;
            self.trail.push(current);
        } else {
            self.swap_last(current);
        }
        self.draw(radius);
    }
}
