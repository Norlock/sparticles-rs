use macroquad::prelude::*;

use crate::point::Point;

#[derive(Debug, Clone)]
pub struct TrailHandler {
    trail: Vec<Point>,
    /// How many iterations of update_ms.
    trail_length: usize,
    /// When to update trail, (lower == ^ precision / higher == ^ performance).
    update_ms: u128,
    iteration: usize,
}

impl TrailHandler {
    pub fn new(trail_length: usize, update_ms: u128) -> Self {
        Self {
            update_ms,
            iteration: 0,
            trail_length,
            trail: Vec::new(),
        }
    }

    fn draw(&self, radius: f32, color: &Color) {
        let diameter = radius * 1.5;
        let until = self.trail.len();
        let mut trail_color = color.clone();

        for i in 1..until {
            let from = &self.trail[i - 1];
            let to = &self.trail[i];
            let fraction = i as f32 / until as f32;
            let line_radius = fraction * diameter;
            trail_color.a = fraction / 1.5;
            draw_line(from.0, from.1, to.0, to.1, line_radius, trail_color);
        }
    }

    pub fn update(&mut self, current: Point, radius: f32, color: &Color, elapsed_ms: u128) {
        let new_iteration = elapsed_ms / self.update_ms;

        if self.iteration == new_iteration as usize {
            self.trail.pop();
            self.trail.push(current);
            self.draw(radius, color);
            return;
        }

        self.iteration = new_iteration as usize;

        if self.trail_length == self.trail.len() {
            self.trail.drain(0..1);
        }

        self.trail.push(current);
        self.draw(radius, color);
    }
}
