use crate::force::Force;
use crate::particle::Particle;

pub struct NewtonForce {
    pub nx: f32,
    pub ny: f32,
    pub max_x: f32,
    pub max_y: f32,
    pub from_ms: u128,
    pub until_ms: u128,
}

impl Force for NewtonForce {
    fn apply(&self, particle: &mut Particle, force_cycle_ms: u128) {
        if force_cycle_ms < self.from_ms || self.until_ms <= force_cycle_ms {
            return;
        }

        let acceleration = ((force_cycle_ms - self.from_ms) as f32 / 1000.).powf(2.);
        // TODO incorporate max_x, max_y
        let vx = self.nx / particle.mass * acceleration;
        let vy = self.ny / particle.mass * acceleration;

        //println!("{} {} min: {}", vx, self.max_x, vx.max(self.max_x));
        //if 0. <= vx {
        //vx = vx.min(self.max_x);
        //} else {
        //vx = vx.max(self.max_x);
        //}

        //if 0. <= vy {
        //vy = vy.min(self.max_y);
        //} else {
        //vy = vy.max(self.max_y);
        //}

        if 0. <= vx && 0. <= particle.vx {
            particle.vx = vx.max(particle.vx);
        } else if vx <= 0. && particle.vx <= 0. {
            particle.vx = vx.min(particle.vx);
        } else {
            particle.vx += vx;
        }

        if 0. <= vy && 0. <= particle.vy {
            particle.vy = vy.max(particle.vy);
        } else if vy <= 0. && particle.vy <= 0. {
            particle.vy = vy.min(particle.vy);
        } else {
            particle.vy += vy;
        }
    }
}
