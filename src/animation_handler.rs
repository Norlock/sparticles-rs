use crate::animation::{Animate, AnimationData};
use macroquad::prelude::rand;
use std::rc::Rc;
use std::time::Instant;

#[derive(Debug)]
pub struct AnimationHandler {
    pub lifetime: Instant,
    animation_offset_ms: u64,
    iteration: u128,
    animations: Rc<Vec<Box<dyn Animate>>>,
    duration_ms: u128,
}

pub enum StartAnimationAt {
    Zero,
    Random,
    RangeMs(u64, u64),
}

pub struct AnimationOptions {
    pub animations: Rc<Vec<Box<dyn Animate>>>,
    pub duration_ms: u128,
    pub start_at: StartAnimationAt,
}

impl AnimationHandler {
    pub fn new(options: &Option<AnimationOptions>) -> Option<Self> {
        if let Some(animation_handler) = options {
            let animation_offset_ms = match animation_handler.start_at {
                StartAnimationAt::Zero => 0,
                StartAnimationAt::Random => {
                    rand::gen_range(0, animation_handler.duration_ms as u64)
                }
                StartAnimationAt::RangeMs(start, end) => rand::gen_range(start, end),
            };
            Some(AnimationHandler {
                lifetime: Instant::now(),
                iteration: 0,
                animation_offset_ms,
                animations: Rc::clone(&animation_handler.animations),
                duration_ms: animation_handler.duration_ms,
            })
        } else {
            None
        }
    }

    pub fn animate(&mut self, data: &mut AnimationData) {
        let elapsed_time_ms = self.lifetime.elapsed().as_millis();
        let new_iteration = self.duration_ms / elapsed_time_ms;

        if self.iteration < new_iteration {
            self.iteration = new_iteration;
        }

        let animation_cycle_ms =
            (elapsed_time_ms + self.animation_offset_ms as u128) % self.duration_ms;

        for animation in self.animations.iter() {
            animation.animate(data, animation_cycle_ms);
        }
    }
}

impl AnimationOptions {
    pub fn new(
        duration_ms: u128,
        start_at: StartAnimationAt,
        animations: Vec<Box<dyn Animate>>,
    ) -> Self {
        Self {
            duration_ms,
            start_at,
            animations: Rc::new(animations),
        }
    }
}
