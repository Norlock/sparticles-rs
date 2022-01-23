use crate::force::Force;
use crate::particle::Particle;

pub struct GravityForce {
    pub center_x: f32,
    pub center_y: f32,
    pub gravitation_force_n: f32,
    /// Use to exclude extreme gravitational pulls, e.g. 20.
    pub dead_zone: f32,
    pub mass: f32,
    pub from_ms: u128,
    pub until_ms: u128,
}

impl Force for GravityForce {
    // Based on newton's law of universal gravity.
    fn apply(&self, particle: &mut Particle, force_cycle_ms: u128) {
        if force_cycle_ms < self.from_ms || self.until_ms <= force_cycle_ms {
            return;
        }

        let particle_center_x = particle.x + particle.diameter / 2.;
        let particle_center_y = particle.y + particle.diameter / 2.;
        let x_distance = self.center_x - particle_center_x;
        let y_distance = self.center_y - particle_center_y;

        if x_distance.abs() < self.dead_zone && y_distance.abs() < self.dead_zone {
            return;
        }

        let x_distance_pow = x_distance.powi(2);
        let y_distance_pow = y_distance.powi(2);
        let distance_pow = x_distance_pow + y_distance_pow;

        let top_formula = self.gravitation_force_n * self.mass * particle.mass;
        let force = top_formula / distance_pow;

        let x_percentage = x_distance_pow / distance_pow;
        let y_percentage = y_distance_pow / distance_pow;

        let vx = force * x_percentage / particle.mass;
        if 0. < x_distance {
            particle.vx += vx;
        } else if x_distance < 0. {
            particle.vx -= vx;
        }

        let vy = force * y_percentage / particle.mass;
        if 0. < y_distance {
            particle.vy += vy;
        } else if y_distance < 0. {
            particle.vy -= vy;
        }
    }
}
