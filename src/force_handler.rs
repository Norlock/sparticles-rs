use crate::particle::Particle;
use crate::Force;
use std::time::Duration;
use std::time::Instant;

#[derive(Debug)]
pub struct ForceHandler {
    pub iteration: u128,
    pub duration_ms: u128,
    pub forces: Vec<Box<dyn Force>>,
    pub elapsed_time_ms: u128,
}

impl ForceHandler {
    pub fn new(duration: Duration) -> Self {
        Self {
            iteration: 0,
            duration_ms: duration.as_millis(),
            forces: Vec::new(),
            elapsed_time_ms: 0,
        }
    }

    pub fn add(&mut self, force: Box<dyn Force>) {
        self.forces.push(force);
    }

    pub fn apply(&self, particle: &mut Particle) {
        for force in self.forces.iter() {
            force.apply(particle, self.elapsed_time_ms);
        }
    }

    pub fn update(&mut self, lifetime: &Instant) {
        let elapsed_time = lifetime.elapsed().as_millis();
        let new_iteration = elapsed_time / self.duration_ms;

        if self.iteration < new_iteration {
            self.iteration = new_iteration;
        }

        self.elapsed_time_ms = elapsed_time % self.duration_ms
    }
}
