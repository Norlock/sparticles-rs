use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct TrailHandler {
    trail: Vec<ColorPoint>,
    /// How many iterations of update_ms.
    trail_length: usize,
    /// When to update trail, (lower == ^ precision / higher == ^ performance).
    update_ms: u128,
    iteration: i16,
}

#[derive(Debug, Clone, Copy)]
pub struct ColorPoint {
    pub x: f32,
    pub y: f32,
    pub color: Color,
}

impl TrailHandler {
    pub fn new(trail_length: usize, update_ms: u128) -> Self {
        Self {
            update_ms,
            iteration: -1,
            trail_length,
            trail: Vec::new(),
        }
    }

    fn draw(&mut self, radius: f32) {
        let diameter = radius * 1.8;
        let until = self.trail.len();

        let mut i = 1;
        self.trail.iter_mut().reduce(|from, to| {
            let fraction = i as f32 / until as f32;
            i += 1;
            to.color.a /= 1.1;
            draw_line(from.x, from.y, to.x, to.y, fraction * diameter, to.color);
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
