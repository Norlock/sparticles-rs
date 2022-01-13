use macroquad::prelude::*;
use std::time::{Duration, Instant};

use crate::position::Position;

pub struct EmitterOptions {
    pub emitter_position: Position,
    pub emitter_diameter: f32,
    pub emitter_duration: Duration,
    pub angle_degrees: f32,
    pub diffusion_degrees: f32,
    pub particle_color: Color,
    pub particles_per_frame: u32,
    pub particle_lifetime: Duration,
    pub particle_radius: f32,
    pub particle_speed: f32,
}

#[derive(Debug)]
pub struct Emitter {
    emitter_diameter: f32,
    start_x: f32,
    start_y: f32,
    end_x: f32,
    end_y: f32,
    angle_radians: f32,
    diffusion_degrees: f32,
    particle_color: Color,
    particles_per_frame: u32,
    particle_lifetime: Duration,
    particle_radius: f32,
    particle_speed: f32,
    particles: Vec<EmittedParticle>,
    pub lifetime: Instant,
    pub emitter_duration: Duration,
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
        let angle_radians = options.angle_degrees.to_radians();
        let start_x = options.emitter_position.x + grid_position.x;
        let start_y = options.emitter_position.y + grid_position.y;
        let end_x = start_x + options.emitter_diameter * angle_radians.cos();
        let end_y = start_y + options.emitter_diameter * angle_radians.sin();

        Self {
            particles_per_frame: options.particles_per_frame,
            particles: Vec::new(),
            particle_color: options.particle_color,
            diffusion_degrees: options.diffusion_degrees,
            particle_speed: options.particle_speed,
            particle_radius: options.particle_radius,
            start_x,
            start_y,
            end_x,
            end_y,
            particle_lifetime: options.particle_lifetime,
            emitter_diameter: options.emitter_diameter,
            angle_radians: options.angle_degrees,
            emitter_duration: options.emitter_duration,
            lifetime: Instant::now(),
        }
    }

    pub fn emit(&mut self) {
        for _ in 0..self.particles_per_frame {
            self.particles.push(self.create_particle());
        }

        self.particles
            .retain(|particle| particle.lifetime.elapsed() <= self.particle_lifetime);

        for particle in self.particles.iter_mut() {
            draw_circle(particle.x, particle.y, particle.radius, self.particle_color);

            particle.x += particle.vx;
            particle.y += particle.vy;
        }
    }

    fn create_particle(&self) -> EmittedParticle {
        let x = rand::gen_range(self.start_x, self.end_x);
        let y = rand::gen_range(self.start_y, self.end_y);

        let diffusion_delta =
            rand::gen_range(-self.diffusion_degrees, self.diffusion_degrees).to_radians();

        let angle = self.angle_radians + diffusion_delta;

        let vx = self.particle_speed * angle.cos();
        let vy = self.particle_speed * angle.sin();

        EmittedParticle {
            x,
            y,
            lifetime: Instant::now(),
            vx,
            vy,
            radius: self.particle_radius,
        }
    }
}
