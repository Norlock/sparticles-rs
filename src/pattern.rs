use crate::constant_force::ConstantForce;
use crate::gravitational_force::GravitationalForce;
use crate::point::Point;
use crate::size_animation::SizeAnimation;
use crate::stray_animation::{self, StrayAnimation};
use crate::swarm_emitter::SwarmEmitter;
use crate::trail_handler::{self, TrailHandler};
use std::rc::Rc;
use std::time::Duration;

use crate::accelerating_force::AcceleratingForce;
use crate::animation_handler::{AnimationOptions, StartAnimationAt};
use crate::animator::Animator;
use crate::color_animation::ColorAnimation;
use crate::emitter::EmitterOptions;
use crate::force_handler::ForceHandler;
use crate::position::Position;
use macroquad::miniquad::Context;
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

    let mut force_handler = ForceHandler::new(Duration::from_secs(4));

    force_handler.add(Box::new(ConstantForce {
        from_ms: 0,
        until_ms: 4000,
        nx: 0.021,
        ny: 0.02,
        max_vx: 2.,
        max_vy: 2.,
    }));

    force_handler.add(Box::new(ConstantForce {
        from_ms: 2000,
        until_ms: 2700,
        nx: 0.,
        ny: -0.03,
        max_vx: 0.,
        max_vy: -2.,
    }));

    let trail_handler = TrailHandler::new(30, 50);
    EmitterOptions {
        emitter_position: Position::new(300., 300.),
        emitter_diameter: 100.,
        emitter_duration: Duration::from_secs(10),
        angle_degrees: 135.,
        emission_distortion_px: 0.,
        trail_handler: Some(trail_handler),
        delay_between_emission: Duration::from_millis(3500),
        diffusion_degrees: 360.,
        particle_color: Color::from_rgba(200, 100, 1, 255),
        particle_texture: None,
        particles_per_emission: 200,
        particle_lifetime: Duration::from_secs(3),
        particle_radius: 5.,
        particle_mass: 1.,
        particle_speed: 2.2,
        particle_friction_coefficient: 0.01,
        respect_grid_bounds: true,
        animations: vec![color_animation, size_animation],
        force_handler: Some(force_handler),
    }
}

pub async fn another_emitter() -> EmitterOptions {
    let color_animation1 = Box::new(ColorAnimation {
        color1: Color::from_rgba(10, 0, 250, 255),
        color2: Color::from_rgba(200, 0, 0, 255),
        from_ms: 0,
        until_ms: 500,
    });
    let color_animation2 = Box::new(ColorAnimation {
        color1: Color::from_rgba(200, 0, 0, 255),
        color2: Color::from_rgba(232, 232, 0, 255),
        from_ms: 500,
        until_ms: 3_000,
    });

    let stray_animation = Box::new(StrayAnimation::new(1_000, 3_000, 10.));

    let mut force_handler = ForceHandler::new(Duration::from_secs(10));
    force_handler.add(Box::new(GravitationalForce {
        from_ms: 0,
        until_ms: 5000,
        gravitation_force: -0.3,
        dead_zone: 30.,
        mass: 1000.,
        start: Point(400., 400.),
        end: Point(400., 800.),
    }));

    force_handler.add(Box::new(GravitationalForce {
        from_ms: 5000,
        until_ms: 10000,
        gravitation_force: -0.4,
        dead_zone: 20.,
        mass: 1000.,
        start: Point(400., 800.),
        end: Point(400., 400.),
    }));

    let trail_handler = TrailHandler::new(10, 64);

    //let texture: Texture2D = load_texture("assets/bubble.png").await.unwrap();

    EmitterOptions {
        emitter_position: Position::new(300., 200.),
        emitter_diameter: 100.,
        emitter_duration: Duration::from_secs(10),
        angle_degrees: 135.,
        emission_distortion_px: 3.,
        trail_handler: Some(trail_handler),
        delay_between_emission: Duration::from_millis(100),
        diffusion_degrees: 60.,
        particle_color: Color::from_rgba(10, 0, 250, 255),
        particle_texture: None,
        particles_per_emission: 20,
        particle_lifetime: Duration::from_secs(3),
        particle_radius: 3.,
        particle_mass: 1.,
        particle_friction_coefficient: 0.007,
        particle_speed: 2.5,
        respect_grid_bounds: true,
        animations: vec![color_animation1, color_animation2, stray_animation],
        force_handler: Some(force_handler),
    }
}

pub fn shimmer_forces() -> Option<ForceHandler> {
    let mut force_handler = ForceHandler::new(Duration::from_secs(6));

    force_handler.add(Box::new(AcceleratingForce {
        from_ms: 0,
        until_ms: 1000,
        nx: 0.11,
        ny: 0.1,
        max_vx: 2.,
        max_vy: 2.,
    }));

    force_handler.add(Box::new(AcceleratingForce {
        from_ms: 2_000,
        until_ms: 3_000,
        nx: 0.1,
        ny: -0.11,
        max_vx: -2.,
        max_vy: -2.,
    }));

    force_handler.add(Box::new(AcceleratingForce {
        from_ms: 3_000,
        until_ms: 4_000,
        nx: -0.1,
        ny: -0.11,
        max_vx: -2.,
        max_vy: -2.,
    }));

    force_handler.add(Box::new(GravitationalForce {
        from_ms: 0,
        until_ms: 6000,
        gravitation_force: 0.5,
        dead_zone: 30.,
        mass: 1000.,
        start: Point(200., 200.),
        end: Point(1000., 1000.),
    }));

    force_handler.add(Box::new(GravitationalForce {
        from_ms: 0,
        until_ms: 6000,
        gravitation_force: 0.4,
        dead_zone: 20.,
        mass: 1000.,
        start: Point(100., 900.),
        end: Point(100., 900.),
    }));

    Some(force_handler)
}

pub fn boid() {
    let flight_pattern = vec![Point(100., 400.), Point(400., 400.), Point(100., 100.)];
    //let emitter = SwarmEmitter {
    //boid_speed: 1.,
    //boid_count: 100,
    //boid_color: Color::from_rgba(0, 255, 0, 255),
    //boid_radius: 5.,
    //emission_delay_ms: 10,
    //diffusion: 0.,
    //flight_pattern,
    //boids: Vec::new(),
    //};
}
