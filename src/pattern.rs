use crate::animation::AnimationData;
use crate::animator::Animator;
use crate::force::{Force, ForceType};
use crate::force_builder::ForceBuilder;

pub fn shimmer() -> Animator {
    let mut animator = Animator::new(900);

    animator.add_time_based(shimmer_out_animation, 0, 100);
    animator.add_time_based(shimmer_in_animation, 700, 900);

    animator
}

fn shimmer_out_animation(data: &mut AnimationData) {
    data.color.a -= 0.01;
}

fn shimmer_in_animation(data: &mut AnimationData) {
    data.color.a += 0.005;
}

pub fn shimmer_forces() -> Vec<Force> {
    let mut builder = ForceBuilder::new();

    builder.add(
        ForceType::Static {
            vx: -0.002,
            vy: -0.001,
        },
        100,
    );

    builder.add(ForceType::None, 200);
    builder.add(
        ForceType::Static {
            vx: 0.002,
            vy: 0.001,
        },
        300,
    );
    builder.add(
        ForceType::Static {
            vx: -0.002,
            vy: -0.002,
        },
        100,
    );

    builder.build()
}
