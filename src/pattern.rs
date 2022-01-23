use crate::boid_emitter::BoidEmitter;
use crate::boid_emitter::Point;
use crate::gravity_force::GravityForce;
use crate::size_animation::SizeAnimation;
use std::rc::Rc;
use std::time::Duration;

use crate::animation_handler::{AnimationOptions, StartAnimationAt};
use crate::animator::Animator;
use crate::color_animation::ColorAnimation;
use crate::emitter::EmitterOptions;
use crate::force_handler::ForceHandler;
use crate::newton_force::NewtonForce;
use crate::position::Position;
use macroquad::prelude::*;

pub fn shimmer_animations() -> AnimationOptions {
    let mut animator = Animator {
        animations: Vec::new(),
        duration_ms: 2000,
    };

    animator.add(Box::new(ColorAnimation {
        color1: Color::from_rgba(255, 255, 255, 255),
        color2: Color::from_rgba(255, 255, 255, 0),
        from_ms: 0,
        until_ms: 1000,
    }));

    animator.add(Box::new(ColorAnimation {
        color1: Color::from_rgba(255, 255, 255, 0),
        color2: Color::from_rgba(255, 255, 255, 255),
        from_ms: 1000,
        until_ms: 2000,
    }));

    //animator.add(Box::new(SizeAnimation {
    //from_ms: 0,
    //until_ms: 750,
    //start_radius: 2.5,
    //end_radius: 1.,
    //}));

    //animator.add(Box::new(SizeAnimation {
    //from_ms: 750,
    //until_ms: 1500,
    //start_radius: 1.,
    //end_radius: 2.5,
    //}));

    AnimationOptions {
        animator: Rc::new(animator),
        start_at: StartAnimationAt::RangeMs(0, 1000),
        //start_at: StartAnimationAt::Zero,
    }
}

pub fn smoke() -> EmitterOptions {
    let color_animation = Box::new(ColorAnimation {
        color1: Color::from_rgba(200, 100, 1, 255),
        color2: Color::from_rgba(145, 42, 245, 255),
        from_ms: 0,
        until_ms: 2000,
    });

    let size_animation = Box::new(SizeAnimation {
        from_ms: 0,
        until_ms: 1000,
        start_radius: 5.,
        end_radius: 2.,
    });

    EmitterOptions {
        emitter_position: Position::new(300., 300.),
        emitter_diameter: 100.,
        emitter_duration: Duration::from_secs(10),
        angle_degrees: 135.,
        emission_distortion_px: 0.,
        delay_between_emission: Duration::from_millis(500),
        diffusion_degrees: 360.,
        particle_color: Color::from_rgba(200, 100, 1, 255),
        particles_per_emission: 200,
        particle_lifetime: Duration::from_secs(2),
        particle_radius: 5.,
        particle_mass: 10.,
        particle_force: 22.,
        particle_friction_coefficient: 0.01,
        respect_grid_bounds: true,
        animations: vec![color_animation, size_animation],
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
        respect_grid_bounds: true,
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

pub fn shimmer_forces() -> Option<ForceHandler> {
    let mut force_handler = ForceHandler::new(Duration::from_secs(6));

    force_handler.add(Box::new(NewtonForce {
        from_ms: 0,
        until_ms: 1000,
        nx: 0.11,
        ny: 0.1,
        max_vx: 2.,
        max_vy: 2.,
    }));

    force_handler.add(Box::new(NewtonForce {
        from_ms: 3_000,
        until_ms: 4_000,
        nx: -0.1,
        ny: -0.11,
        max_vx: -2.,
        max_vy: -2.,
    }));

    //force_handler.add(Box::new(NewtonForce {
    //from_ms: 3000,
    //until_ms: 3500,
    //nx: 1.1,
    //ny: -1.,
    //max_x: -3.,
    //max_y: -3.,
    //}));

    //force_handler.add(Box::new(NewtonForce {
    //from_ms: 3500,
    //until_ms: 4000,
    //nx: -1.,
    //ny: 1.1,
    //max_x: -3.,
    //max_y: -3.,
    //}));

    force_handler.add(Box::new(GravityForce {
        from_ms: 0,
        until_ms: u128::MAX,
        center_x: 500.,
        center_y: 500.,
        gravitation_force_n: 0.3,
        dead_zone: 20.,
        mass: 1000.,
    }));

    //force_handler.add(Box::new(GravityForce {
    //from_ms: 1500,
    //until_ms: 1700,
    //center_x: 500.,
    //center_y: 500.,
    //gravitation_force_n: -1.,
    //dead_zone: 20.,
    //mass: 1000.,
    //}));
    Some(force_handler)
}

pub fn boid() {
    let flight_pattern = vec![Point(100., 400.), Point(400., 400.), Point(100., 100.)];
    let emitter = BoidEmitter {
        boid_speed: 1.,
        boid_count: 100,
        boid_color: Color::from_rgba(0, 255, 0, 255),
        boid_radius: 5.,
        emission_delay_ms: 10,
        diffusion: 0.,
        flight_pattern,
        boids: Vec::new(),
    };
}
