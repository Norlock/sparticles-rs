use std::rc::Rc;
use std::time::Duration;

use crate::animation::AnimationData;
use crate::animation_handler::{AnimationOptions, StartAnimationAt};
use crate::animator::Animator;
use crate::color_animation::ColorAnimation;
use crate::emitter::EmitterOptions;
use crate::force::{Force, ForceType};
use crate::force_builder::ForceBuilder;
use crate::position::Position;
use macroquad::prelude::*;

pub fn shimmer_animations() -> AnimationOptions {
    let mut animator = Animator {
        animations: Vec::new(),
        duration_ms: 2000,
    };

    animator.add(Box::new(ColorAnimation {
        color1: Color::from_rgba(255, 255, 0, 255),
        color2: Color::from_rgba(255, 255, 0, 0),
        from_ms: 0,
        until_ms: 1000,
    }));

    animator.add(Box::new(ColorAnimation {
        color1: Color::from_rgba(255, 255, 0, 0),
        color2: Color::from_rgba(255, 255, 0, 255),
        from_ms: 1000,
        until_ms: 2000,
    }));

    AnimationOptions {
        animator: Rc::new(animator),
        start_at: StartAnimationAt::RangeMs(0, 500),
    }
}

fn shimmer_out_animation(data: &mut AnimationData) {
    data.color.a -= 0.01;
}

fn shimmer_in_animation(data: &mut AnimationData) {
    data.color.a += 0.005;
}

pub fn smoke() -> EmitterOptions {
    let color_animation = ColorAnimation {
        color1: Color::from_rgba(200, 100, 1, 255),
        color2: Color::from_rgba(145, 42, 245, 255),
        from_ms: 0,
        until_ms: 2500,
    };

    EmitterOptions {
        emitter_position: Position::new(200., 200.),
        emitter_diameter: 100.,
        emitter_duration: Duration::from_secs(10),
        angle_degrees: 135.,
        emission_distortion_px: 0.,
        delay_between_emission: Duration::from_millis(500),
        diffusion_degrees: 360.,
        particle_color: Color::from_rgba(200, 100, 1, 255),
        particles_per_emission: 200,
        particle_lifetime: Duration::from_secs(2),
        particle_radius: 3.,
        particle_mass: 10.,
        particle_force: 22.,
        particle_friction_coefficient: 0.01,
        respect_grid_bounds: true,
        animations: vec![Box::new(color_animation)],
    }
}

pub fn another_emitter() -> EmitterOptions {
    let color_animation = ColorAnimation {
        color2: Color::from_rgba(212, 132, 64, 255),
        color1: Color::from_rgba(2, 200, 1, 255),
        from_ms: 200,
        until_ms: 700,
    };

    EmitterOptions {
        emitter_position: Position::new(300., 200.),
        emitter_diameter: 100.,
        emitter_duration: Duration::from_secs(10),
        angle_degrees: 135.,
        emission_distortion_px: 0.,
        delay_between_emission: Duration::from_secs(1),
        diffusion_degrees: 45.,
        particle_color: Color::from_rgba(2, 200, 1, 255),
        particles_per_emission: 100,
        particle_lifetime: Duration::from_secs(2),
        particle_radius: 3.,
        particle_mass: 1.,
        particle_friction_coefficient: 0.008,
        particle_force: 2.5,
        respect_grid_bounds: false,
        animations: vec![Box::new(color_animation)],
    }
    //EmitterOptions {
    //emitter_position: Position::new(500., 500.),
    //emitter_diameter: 100.,
    //emitter_duration: Duration::from_secs(10),
    //angle_degrees: 135.,
    //emission_distortion_px: 0.,
    //frames_per_emission: 100,
    //diffusion_degrees: 270.,
    //particle_color: Color::from_rgba(20, 200, 200, 255),
    //particles_per_emission: 50,
    //particle_lifetime: Duration::from_secs(2),
    //particle_radius: 3.,
    //particle_speed: 1.,
    //respect_grid_bounds: true,
    //}
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
