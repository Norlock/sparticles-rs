use macroquad::prelude::*;
use std::time::{Duration, Instant};

use crate::position::Position;

pub struct EmitterOptions {
    pub emitter_position: Position,
    pub emitter_diameter: f32,
    pub emitter_lifetime: Duration,
    pub angle: f32,
    pub diffusion: u32,
    pub particle_color: Color,
    pub particles_per_frame: u32,
    pub particle_lifetime: Duration,
    pub particle_radius: f32,
    pub particle_speed: f32,
}

#[derive(Debug)]
pub struct Emitter {
    grid_position: Position,
    emitter_diameter: f32,
    emitter_position: Position,
    emitter_lifetime: Duration,
    angle: f32,
    diffusion: u32,
    particle_color: Color,
    particles_per_frame: u32,
    particle_lifetime: Duration,
    particle_radius: f32,
    particle_speed: f32,
    particles: Vec<EmittedParticle>,
    instant: Instant,
}

#[derive(Debug)]
struct EmittedParticle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    radius: f32,
    lifetime: Instant,
}

impl Emitter {
    pub fn new(grid_position: &Position, options: EmitterOptions) -> Self {
        Self {
            particles_per_frame: options.particles_per_frame,
            particles: Vec::new(),
            grid_position: grid_position.clone(),
            particle_color: options.particle_color,
            diffusion: options.diffusion,
            particle_speed: options.particle_speed,
            particle_radius: options.particle_radius,
            emitter_position: options.emitter_position,
            particle_lifetime: options.particle_lifetime,
            emitter_diameter: options.emitter_diameter,
            angle: options.angle,
            emitter_lifetime: options.emitter_lifetime,
            instant: Instant::now(),
        }
    }

    pub fn emit(&mut self) {
        for _ in 0..self.particles_per_frame {
            self.particles.push(self.create_particle());
        }

        self.particles
            .retain(|particle| particle.lifetime.elapsed() <= self.particle_lifetime);

        for particle in self.particles.iter_mut() {
            draw_circle(
                particle.x + self.grid_position.x,
                particle.y + self.grid_position.y,
                particle.radius,
                self.particle_color,
            );

            particle.x += particle.vx;
            particle.y += particle.vy;
        }
    }

    fn create_particle(&self) -> EmittedParticle {
        // line is
        let start_x = self.emitter_position.x;
        let start_y = self.emitter_position.y;

        let end_x = start_x + self.emitter_diameter * self.angle.cos();
        let end_y = start_y + self.emitter_diameter * self.angle.sin();

        let x = rand::gen_range(start_x, end_x);
        let y = rand::gen_range(start_y, end_y);

        EmittedParticle {
            x,
            y,
            lifetime: Instant::now(),
            vx: 1.,
            vy: 1.,
            radius: self.particle_radius,
        }
    }
}
