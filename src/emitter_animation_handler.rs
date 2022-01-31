use crate::emitter_animation::EmitterAnimate;

#[derive(Debug)]
pub struct EmitterAnimationHandler {
    iteration: u16,
    animations: Vec<Box<dyn EmitterAnimate>>,
    duration_ms: u128,
}

impl EmitterAnimationHandler {
    pub fn new(duration_ms: u128, animations: Vec<Box<dyn EmitterAnimate>>) -> Self {
        Self {
            animations,
            duration_ms,
            iteration: 0,
        }
    }
}
