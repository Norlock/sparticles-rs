use crate::animation::AnimationData;
use crate::animator::Animator;
use macroquad::prelude::rand;
use std::rc::Rc;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct AnimationHandler {
    pub lifetime: Instant,
    animation_offset_ms: u64,
    iteration: u128,
    animator: Rc<Animator>,
}

pub enum StartAnimationAt {
    Zero,
    Random,
    RangeMs(u64, u64),
}

pub struct AnimationOptions {
    pub animator: Rc<Animator>,
    pub start_at: StartAnimationAt,
}

impl AnimationHandler {
    pub fn new(options: &Option<AnimationOptions>) -> Option<Self> {
        if let Some(animation_handler) = options {
            let start_frame = match animation_handler.start_at {
                StartAnimationAt::Zero => 0,
                StartAnimationAt::Random => {
                    rand::gen_range(0, animation_handler.animator.duration_ms as u64)
                }
                StartAnimationAt::RangeMs(start, end) => rand::gen_range(start, end),
            };
            Some(AnimationHandler {
                lifetime: Instant::now(),
                animator: animation_handler.animator.clone(),
                iteration: 0,
                animation_offset_ms: start_frame,
            })
        } else {
            None
        }
    }

    pub fn animate(&mut self, data: &mut AnimationData) {
        let elapsed_time_ms = self.lifetime.elapsed().as_millis();
        let new_iteration = self.animator.duration_ms / elapsed_time_ms;

        if self.iteration < new_iteration {
            self.iteration = new_iteration;
        }

        let animation_cycle_ms =
            (elapsed_time_ms + self.animation_offset_ms as u128) % self.animator.duration_ms;

        for animation in self.animator.animations.iter() {
            animation.animate(data, animation_cycle_ms);
        }
    }
}
