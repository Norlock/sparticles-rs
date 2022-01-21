use crate::animation::Animate;

#[derive(Debug)]
pub struct Animator {
    pub animations: Vec<Box<dyn Animate>>,
    pub duration_ms: u128,
}

impl Animator {
    pub fn add(&mut self, animation: Box<dyn Animate>) {
        self.animations.push(animation);
    }
}
