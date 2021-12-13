use crate::particle::Particle;

pub struct Transform {
    pub vx: f32,
    pub vy: f32,
}

impl Transform {
    pub fn new(particle: &Particle) -> Self {
        Self {
            vx: particle.vx,
            vy: particle.vy,
        }
    }
}
