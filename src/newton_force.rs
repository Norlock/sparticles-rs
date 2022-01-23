use crate::force::Force;
use crate::particle::Particle;

pub struct NewtonForce {
    pub nx: f32,
    pub ny: f32,
    pub max_vx: f32,
    pub max_vy: f32,
    pub from_ms: u128,
    pub until_ms: u128,
}

const MS_PER_SEC: f32 = 1000.;

impl Force for NewtonForce {
    fn apply(&self, particle: &mut Particle, force_cycle_ms: u128) {
        if force_cycle_ms < self.from_ms || self.until_ms <= force_cycle_ms {
            return;
        }

        let acceleration = ((force_cycle_ms - self.from_ms) as f32 / MS_PER_SEC).powf(2.);
        let vx = self.nx * acceleration / particle.mass;
        let vy = self.ny * acceleration / particle.mass;

        let new_vx = particle.vx + vx;
        let new_vy = particle.vy + vy;

        if 0. < vx && 0. <= particle.vx && new_vx <= self.max_vx {
            particle.vx += vx;
        } else if vx < 0. && particle.vx <= 0. && self.max_vx <= new_vx {
            particle.vx += vx;
        } else {
            particle.vx += vx;
        }

        if 0. < vy && 0. <= particle.vy && new_vy <= self.max_vy {
            particle.vy += vy;
        } else if vy < 0. && particle.vy <= 0. && self.max_vy <= new_vy {
            particle.vy += vy;
        } else {
            particle.vy += vy;
        }
    }
}
