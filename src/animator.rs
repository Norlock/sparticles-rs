use crate::animation::{Animate, Animation};

#[derive(Debug)]
pub struct Animator {
    pub animations: Vec<Animation>,
    pub until_frame: u32,
}

impl Animator {
    pub fn new(until_frame: u32) -> Self {
        Self {
            until_frame,
            animations: Vec::new(),
        }
    }

    pub fn add_time_based(&mut self, animate: Animate, start: u32, until: u32) {
        let time_based: Animation = Animation::TimeBased {
            start,
            until,
            animate,
        };

        self.animations.push(time_based);
    }

    pub fn add_allways(&mut self, animate: Animate) {
        self.animations.push(Animation::Allways(animate));
    }
}
