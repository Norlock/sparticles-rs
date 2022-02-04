use std::fmt::Debug;
use std::time::Duration;

pub trait EmitterAnimate {
    fn animate(&self, data: &mut EmitterData, animation_cycle_ms: u32);
}

impl Debug for dyn EmitterAnimate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Animate")
    }
}

pub struct EmitterData {
    pub emitter_diameter: f32,
    pub x: f32,
    pub y: f32,
    pub respect_grid_bounds: bool,
    pub angle_radians: f32,
    pub diffusion_radians: f32,
    pub particles_per_emission: u32,
    pub delay_between_emission_ms: u128,
    pub emission_distortion: f32,
    pub particle_lifetime: Duration,
    /// Only on newly spawned particles
    pub particle_radius: f32,
    /// Only on newly spawned particles
    pub particle_mass: f32,
    /// Only on newly spawned particles
    pub particle_speed: f32,
    /// Only on newly spawned particles
    pub particle_friction_coefficient: f32,
}
