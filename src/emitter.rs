use crate::{force::ForceData, force_handler::ForceHandler};
use macroquad::prelude::*;
use std::{
    f64::consts::PI,
    time::{Duration, Instant},
};

use crate::{
    animation::{Animate, AnimationData},
    position::Position,
};

pub struct EmitterOptions {
    pub emitter_position: Position,
    pub emitter_diameter: f32,
    pub emitter_duration: Duration,
    pub angle_degrees: f32,
    /// Initial spread factor
    pub diffusion_degrees: f32,
    /// How well will it stay on course. (0. for perfect).
    pub stray_degrees: f32,
    pub emission_distortion_px: f32,
    pub particle_color: Color,
    pub particles_per_emission: u32,
    pub delay_between_emission: Duration,
    pub particle_lifetime: Duration,
    pub particle_radius: f32,
    pub particle_mass: f32,
    /// Newton force
    pub particle_speed: f32,
    /// number between 0 and 1, e.g. 0.001
    pub particle_friction_coefficient: f32,
    pub respect_grid_bounds: bool,
    pub animations: Vec<Box<dyn Animate>>,
    pub force_handler: Option<ForceHandler>,
}

#[derive(Debug)]
pub struct Emitter {
    emitter_diameter: f32,
    x: f32,
    y: f32,
    grid_position: Position,
    respect_grid_bounds: bool,
    angle_radians: f32,
    angle_emission_radians: f32,
    diffusion_radians: f32,
    stray_radians: f32,
    particle_color: Color,
    particles_per_emission: u32,
    delay_between_emission_ms: u128,
    emission_distortion: f32,
    current_emission: i32,
    particle_lifetime: Duration,
    particle_radius: f32,
    particle_mass: f32,
    particle_speed: f32,
    particle_friction_coefficient: f32,
    particles: Vec<EmittedParticle>,
    lifetime: Instant,
    emitter_duration: Duration,
    animations: Vec<Box<dyn Animate>>,
    force_handler: Option<ForceHandler>,
    pub delete: bool,
}

#[derive(Debug)]
struct EmittedParticle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    radius: f32,
    lifetime: Instant,
    color: Color,
}

impl Emitter {
    pub fn new(grid_position: &Position, options: EmitterOptions) -> Self {
        let angle_radians = options.angle_degrees.to_radians();
        let inverse_radians = (-90. as f32).to_radians(); // 0 deg will be emitting above
        let angle_emission_radians = angle_radians + inverse_radians;
        let x = options.emitter_position.x;
        let y = options.emitter_position.y;

        Self {
            particles_per_emission: options.particles_per_emission,
            particles: Vec::new(),
            particle_color: options.particle_color,
            diffusion_radians: options.diffusion_degrees.to_radians(),
            stray_radians: options.stray_degrees.to_radians(),
            particle_mass: options.particle_mass,
            particle_radius: options.particle_radius,
            x,
            y,
            grid_position: grid_position.clone(),
            angle_radians,
            angle_emission_radians,
            emission_distortion: options.emission_distortion_px,
            particle_lifetime: options.particle_lifetime,
            emitter_diameter: options.emitter_diameter,
            emitter_duration: options.emitter_duration,
            lifetime: Instant::now(),
            current_emission: -1,
            delay_between_emission_ms: options.delay_between_emission.as_millis(),
            respect_grid_bounds: options.respect_grid_bounds,
            particle_friction_coefficient: options.particle_friction_coefficient,
            particle_speed: options.particle_speed,
            animations: options.animations,
            force_handler: options.force_handler,
            delete: false,
        }
    }

    pub fn emit(&mut self) {
        let elapsed = self.lifetime.elapsed();
        let time_elapsed = elapsed > self.emitter_duration;
        let new_emission = (elapsed.as_millis() / self.delay_between_emission_ms) as i32;

        if !time_elapsed && self.current_emission < new_emission {
            self.current_emission = new_emission;
            for _ in 0..self.particles_per_emission {
                self.particles.push(self.create_particle());
            }
        }

        if let Some(force_handler) = &mut self.force_handler {
            force_handler.update(&self.lifetime);
        }

        for i in (0..self.particles.len()).rev() {
            let mut particle = self.particles.swap_remove(i);

            let x_force = particle.vx * self.particle_mass;
            let y_force = particle.vy * self.particle_mass;

            let x_friction = x_force * self.particle_friction_coefficient;
            let y_friction = y_force * self.particle_friction_coefficient;

            let vx = (x_force - x_friction) / self.particle_mass;
            let vy = (y_force - y_friction) / self.particle_mass;

            if let Some(force_handler) = &mut self.force_handler {
                let mut data = ForceData {
                    x: particle.x,
                    y: particle.y,
                    vx,
                    vy,
                    radius: self.particle_radius,
                    mass: self.particle_mass,
                };

                force_handler.apply(&mut data);

                particle.vx = data.vx;
                particle.vy = data.vy;
            } else {
                particle.vx = vx;
                particle.vy = vy;
            }

            let mut anim_data: AnimationData = AnimationData {
                radius: particle.radius,
                color: particle.color,
            };

            for animator in self.animations.iter() {
                animator.animate(&mut anim_data, particle.lifetime.elapsed().as_millis());
            }

            if 0. < self.stray_radians {
                let stray = rand::gen_range(-self.stray_radians, self.stray_radians);
                particle.vx = (particle.vx * stray.cos()) - (particle.vy * stray.sin());
                particle.vy = (particle.vx * stray.sin()) + (particle.vy * stray.cos());
            }

            particle.x += particle.vx;
            particle.y += particle.vy;
            particle.color = anim_data.color;
            particle.radius = anim_data.radius;

            draw_circle(
                particle.x + self.grid_position.x,
                particle.y + self.grid_position.y,
                particle.radius,
                particle.color,
            );

            let diameter = particle.radius * 2.;

            if self.respect_grid_bounds
                && (particle.x < 0.
                    || self.grid_position.width < particle.x + diameter
                    || particle.y < 0.
                    || self.grid_position.height < particle.y + diameter)
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

        let diffusion_delta = rand::gen_range(-self.diffusion_radians, self.diffusion_radians);

        let angle_radians = self.angle_emission_radians + diffusion_delta;
        let vx = self.particle_speed * angle_radians.cos();
        let vy = self.particle_speed * angle_radians.sin();

        EmittedParticle {
            x,
            y,
            lifetime: Instant::now(),
            vx,
            vy,
            radius: self.particle_radius,
            color: self.particle_color,
        }
    }
}
