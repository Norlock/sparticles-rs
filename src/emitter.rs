use macroquad::prelude::*;
use std::time::{Duration, Instant};

use crate::{
    animation::{Animate, AnimationData},
    position::Position,
};

pub struct EmitterOptions {
    pub emitter_position: Position,
    pub emitter_diameter: f32,
    pub emitter_duration: Duration,
    pub angle_degrees: f32,
    pub diffusion_degrees: f32,
    pub emission_distortion_px: f32,
    pub particle_color: Color,
    pub particles_per_emission: u32,
    pub delay_between_emission: Duration,
    pub particle_lifetime: Duration,
    pub particle_radius: f32,
    pub particle_mass: f32,
    /// Newton force
    pub particle_force: f32,
    /// number between 0 and 1, e.g. 0.001
    pub particle_friction_coefficient: f32,
    pub respect_grid_bounds: bool,
    pub animations: Vec<Box<dyn Animate>>,
}

#[derive(Debug)]
pub struct Emitter {
    emitter_diameter: f32,
    x: f32,
    y: f32,
    end_x: f32,
    end_y: f32,
    grid_position: Position,
    respect_grid_bounds: bool,
    angle_radians: f32,
    angle_emission_radians: f32,
    diffusion_degrees: f32,
    particle_color: Color,
    particles_per_emission: u32,
    ms_delay_between_emission: u128,
    emission_distortion: f32,
    current_emission: i32,
    particle_lifetime: Duration,
    particle_radius: f32,
    particle_mass: f32,
    particle_force: f32,
    particle_friction_coefficient: f32,
    particles: Vec<EmittedParticle>,
    lifetime: Instant,
    emitter_duration: Duration,
    animations: Vec<Box<dyn Animate>>,
    pub delete: bool,
}

#[derive(Debug)]
struct EmittedParticle {
    x: f32,
    y: f32,
    x_force: f32,
    y_force: f32,
    radius: f32,
    lifetime: Instant,
    color: Color,
}

impl Emitter {
    pub fn new(grid_position: &Position, options: EmitterOptions) -> Self {
        let angle_radians = options.angle_degrees.to_radians();
        let inverse_radians = (-90. as f32).to_radians(); // 0 deg will be emitting above
        let angle_emission_radians = angle_radians + inverse_radians;
        let x = options.emitter_position.x + grid_position.x;
        let y = options.emitter_position.y + grid_position.y;
        let end_x = grid_position.x + grid_position.width;
        let end_y = grid_position.y + grid_position.height;

        Self {
            particles_per_emission: options.particles_per_emission,
            particles: Vec::new(),
            particle_color: options.particle_color,
            diffusion_degrees: options.diffusion_degrees,
            particle_mass: options.particle_mass,
            particle_radius: options.particle_radius,
            x,
            y,
            end_x,
            end_y,
            grid_position: grid_position.clone(),
            angle_radians,
            angle_emission_radians,
            emission_distortion: options.emission_distortion_px,
            particle_lifetime: options.particle_lifetime,
            emitter_diameter: options.emitter_diameter,
            emitter_duration: options.emitter_duration,
            lifetime: Instant::now(),
            current_emission: -1,
            ms_delay_between_emission: options.delay_between_emission.as_millis(),
            respect_grid_bounds: options.respect_grid_bounds,
            particle_friction_coefficient: options.particle_friction_coefficient,
            particle_force: options.particle_force,
            animations: options.animations,
            delete: false,
        }
    }

    pub fn emit(&mut self) {
        let elapsed = self.lifetime.elapsed();
        let time_elapsed = elapsed > self.emitter_duration;
        let new_emission = (elapsed.as_millis() / self.ms_delay_between_emission) as i32;

        if !time_elapsed && self.current_emission < new_emission {
            self.current_emission = new_emission;
            for _ in 0..self.particles_per_emission {
                self.particles.push(self.create_particle());
            }
        }

        for i in (0..self.particles.len()).rev() {
            let mut particle = self.particles.swap_remove(i);

            let x_friction = particle.x_force * self.particle_friction_coefficient;
            let y_friction = particle.y_force * self.particle_friction_coefficient;

            particle.x_force -= x_friction;
            particle.y_force -= y_friction;

            let vx = particle.x_force / self.particle_mass;
            let vy = particle.y_force / self.particle_mass;

            particle.x += vx;
            particle.y += vy;

            let diameter = particle.radius * 2.;

            let mut data: AnimationData = AnimationData {
                vx,
                vy,
                radius: diameter,
                color: particle.color,
            };

            for animator in self.animations.iter() {
                animator.animate(&mut data, self.lifetime.elapsed().as_millis());
            }

            particle.color = data.color;

            draw_circle(particle.x, particle.y, particle.radius, particle.color);

            if self.respect_grid_bounds
                && (particle.x < self.grid_position.x
                    || self.end_x < particle.x + diameter
                    || particle.y < self.grid_position.y
                    || self.end_y < particle.y + diameter)
            {
                continue; // removes particle.
            }

            if particle.lifetime.elapsed() <= self.particle_lifetime {
                self.particles.push(particle);
            }
        }

        if self.particles.len() == 0 && time_elapsed {
            self.delete = true;
        }
    }

    fn create_particle(&self) -> EmittedParticle {
        let position = rand::gen_range(0., self.emitter_diameter);
        let distortion = rand::gen_range(-self.emission_distortion, self.emission_distortion);
        let x = (self.x + distortion) + position * self.angle_radians.cos();
        let y = (self.y + distortion) + position * self.angle_radians.sin();

        let diffusion_delta =
            rand::gen_range(-self.diffusion_degrees, self.diffusion_degrees).to_radians();

        let angle = self.angle_emission_radians + diffusion_delta;
        let x_force = self.particle_force * angle.cos();
        let y_force = self.particle_force * angle.sin();

        EmittedParticle {
            x,
            y,
            lifetime: Instant::now(),
            x_force,
            y_force,
            radius: self.particle_radius,
            color: self.particle_color,
        }
    }
}
