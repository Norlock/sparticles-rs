use std::f32::consts::PI;
use std::time::Duration;

use crate::animation::AnimationData;
use crate::animator::Animator;
use crate::emitter::EmitterOptions;
use crate::force::{Force, ForceType};
use crate::force_builder::ForceBuilder;
use crate::position::Position;
use macroquad::prelude::*;

pub fn shimmer() -> Animator {
    let mut animator = Animator::new(900);

    animator.add_time_based(shimmer_out_animation, 0, 100);
    animator.add_time_based(shimmer_in_animation, 700, 900);
    //animator.add_allways(move_animation);

    animator
}

fn shimmer_out_animation(data: &mut AnimationData) {
    data.color.a -= 0.01;
}

fn shimmer_in_animation(data: &mut AnimationData) {
    data.color.a += 0.005;
}

pub fn smoke() -> EmitterOptions {
    EmitterOptions {
        emitter_position: Position::new(100., 100.),
        emitter_diameter: 100.,
        emitter_duration: Duration::from_secs(10),
        angle_degrees: 45.,
        diffusion_degrees: 10.,
        particle_color: Color::from_rgba(200, 1, 1, 255),
        particles_per_frame: 30,
        particle_lifetime: Duration::from_secs(2),
        particle_radius: 3.,
        particle_speed: 1.,
    }
}

fn move_animation(data: &mut AnimationData) {
    let test = (data.raw_frame_counter as f32 + rand::gen_range(0., 5.)).sin() / 10.;
    println!("{}", test);
    data.vx += test;
    data.vy += test;
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
        200,
    );

    builder.add(ForceType::None, 200);
    builder.add(
        ForceType::Static {
            vx: -0.002,
            vy: -0.001,
        },
        100,
    );
    //builder.add(
    //ForceType::Static {
    //vx: -0.002,
    //vy: -0.002,
    //},
    //100,
    //);

    builder.build()
}
