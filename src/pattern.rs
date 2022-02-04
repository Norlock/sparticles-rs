use crate::animation::animation::Animate;
use crate::animation::animation_handler::AnimationOptions;
use crate::animation::animation_handler::StartAnimationAt;
use crate::animation::color_animation::ColorAnimation;
use crate::animation::size_animation::SizeAnimation;
use crate::animation::stray_animation::StrayAnimation;
use crate::emitter::emitter::EmitterOptions;
use crate::force::accelerating_force::AcceleratingForce;
use crate::force::constant_force::ConstantForce;
use crate::force::force_handler::ForceHandler;
use crate::force::gravitational_force::GravitationalForce;
use crate::movement_handler::MovementHandler;
use crate::point::Point;
use crate::trail::trail_animation::TrailAnimation;
use crate::trail::trail_animation::TrailOptions;
use crate::trail::trail_handler::TrailHandler;
use std::time::Duration;

use crate::position::Position;
use macroquad::prelude::*;

pub fn shimmer_animations() -> AnimationOptions {
    let mut animations: Vec<Box<dyn Animate>> = Vec::new();

    animations.push(Box::new(ColorAnimation {
        color1: Color::from_rgba(255, 255, 255, 255),
        color2: Color::from_rgba(255, 255, 255, 0),
        from_ms: 1000,
        until_ms: 2000,
    }));

    animations.push(Box::new(ColorAnimation {
        color1: Color::from_rgba(255, 255, 255, 0),
        color2: Color::from_rgba(255, 255, 255, 255),
        from_ms: 3000,
        until_ms: 4000,
    }));

    AnimationOptions::new(4000, StartAnimationAt::RangeMs(0, 1000), animations)
}

pub fn trail_animation() -> TrailHandler {
    let trail_animations = vec![TrailAnimation::new(TrailOptions {
        update_ms: 16,
        opacity_loss_per_update: 1. / 3.,
        diameter_fraction: 0.5,
        from_ms: 0_000,
        until_ms: 10_000,
    })];

    TrailHandler {
        duration_ms: 10_000,
        trail_animations,
    }
}

pub fn smoke() -> EmitterOptions {
    let mut animations: Vec<Box<dyn Animate>> = Vec::new();

    animations.push(Box::new(ColorAnimation {
        color1: Color::from_rgba(200, 100, 1, 255),
        color2: Color::from_rgba(145, 42, 245, 255),
        from_ms: 0,
        until_ms: 2000,
    }));

    animations.push(Box::new(SizeAnimation {
        from_ms: 0,
        until_ms: 1000,
        start_radius: 1.,
        end_radius: 3.,
    }));

    let trail_anim_1 = TrailAnimation::new(TrailOptions {
        update_ms: 32,
        opacity_loss_per_update: 1. / 6.,
        diameter_fraction: 0.7,
        from_ms: 0_000,
        until_ms: 1_000,
    });
    let trail_anim_2 = TrailAnimation::new(TrailOptions {
        update_ms: 32,
        opacity_loss_per_update: 1. / 5.,
        diameter_fraction: 0.9,
        from_ms: 1_700,
        until_ms: 3_330,
    });

    let trail_handler = TrailHandler {
        duration_ms: 4000,
        trail_animations: vec![trail_anim_1, trail_anim_2],
    };

    let animation_options = AnimationOptions::new(4000, StartAnimationAt::Zero, animations);

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

    EmitterOptions {
        emitter_position: Position::new(300., 300.),
        emitter_diameter: 100.,
        emitter_duration: Duration::from_secs(10),
        angle_degrees: 135.,
        emission_distortion_px: 0.,
        delay_between_emission: Duration::from_millis(2500),
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
        particle_animation_options: Some(animation_options),
        force_handler: Some(force_handler),
        emitter_animation_handler: None,
        trail_handler: Some(trail_handler),
    }
}

pub fn another_emitter() -> EmitterOptions {
    let mut animations: Vec<Box<dyn Animate>> = Vec::new();

    animations.push(Box::new(ColorAnimation {
        color1: Color::from_rgba(0, 10, 20, 255),
        color2: Color::from_rgba(0, 61, 152, 255),
        from_ms: 0,
        until_ms: 500,
    }));

    animations.push(Box::new(ColorAnimation {
        color1: Color::from_rgba(0, 61, 162, 255),
        color2: Color::from_rgba(102, 0, 102, 255),
        from_ms: 500,
        until_ms: 3_000,
    }));

    animations.push(Box::new(StrayAnimation::new(1_000, 3_000, 10.)));

    let animation_options = AnimationOptions::new(3_000, StartAnimationAt::Zero, animations);

    let trail_animations = vec![TrailAnimation::new(TrailOptions {
        from_ms: 0,
        until_ms: 3_000,
        update_ms: 32,
        opacity_loss_per_update: 0.1,
        diameter_fraction: 0.7,
    })];

    let trail_handler = Some(TrailHandler {
        duration_ms: 3_000,
        trail_animations,
    });

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

    let movement_handler = MovementHandler::new(1.);

    EmitterOptions {
        emitter_position: Position::new(300., 200.),
        emitter_diameter: 100.,
        emitter_duration: Duration::from_secs(10),
        angle_degrees: 135.,
        emission_distortion_px: 3.,
        delay_between_emission: Duration::from_millis(100),
        diffusion_degrees: 70.,
        particle_color: Color::from_rgba(10, 0, 250, 255),
        particle_texture: None,
        particles_per_emission: 30,
        particle_lifetime: Duration::from_secs(3),
        particle_radius: 3.,
        particle_mass: 1.,
        particle_friction_coefficient: 0.007,
        particle_speed: 2.5,
        respect_grid_bounds: true,
        particle_animation_options: Some(animation_options),
        force_handler: Some(force_handler),
        emitter_animation_handler: None,
        trail_handler,
    }
}

pub fn random_forces() -> Option<ForceHandler> {
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
