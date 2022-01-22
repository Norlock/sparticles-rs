use crate::particle::Particle;
use std::fmt::Debug;

pub trait Force {
    fn apply(&self, particle: &mut Particle, force_cycle_ms: u128);
}

impl Debug for dyn Force {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Force")
    }
}
