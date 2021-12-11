use crate::particle::Particle;
use std::fmt;

type Animate = fn(particle: &mut Particle);

pub struct Animation {
    pub min_frame: u32,
    pub max_frame: u32,
    pub animate: Animate,
}

impl fmt::Debug for Animation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Animation")
            .field("min_frame", &self.min_frame)
            .field("max_frame", &self.max_frame)
            .finish()
    }
}
