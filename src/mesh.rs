use crate::particle::Particle;
use core::fmt::Debug;
use core::fmt::Formatter;

pub trait Mesh {
    fn transform(&mut self, particle: &Particle);
}

impl Debug for dyn Mesh {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        // Nothing
        Ok(())
    }
}
