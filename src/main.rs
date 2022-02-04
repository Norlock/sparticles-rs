#![allow(dead_code)]

mod accelerating_force;
mod animation;
mod animation_handler;
mod collision;
mod color_animation;
mod constant_force;
mod container;
mod emitter;
mod emitter_animation;
mod emitter_animation_handler;
mod fill_style;
mod force;
mod force_handler;
mod forcer;
mod gravitational_force;
mod grid;
mod movement_handler;
mod particle;
mod pattern;
mod point;
mod position;
mod size_animation;
mod stray_animation;
mod swarm_emitter;
mod trail_animation;
mod trail_handler;

use grid::{Grid, GridOptions};

use fill_style::FillStyle;
use force::Force;
use macroquad::prelude::*;
use particle::ParticleAttributes;
use pattern::{another_emitter, shimmer_animations, shimmer_forces, smoke, trail_animation};
use position::Position;

#[macroquad::main("Particles")]
async fn main() {
    let position = Position::new(100., 100.);

    let mut grid = Grid::new(GridOptions {
        cell_x_count: 10,
        cell_y_count: 10,
        possibility_x_count: 10,
        possibility_y_count: 10,
        possibility_side_length: 10,
        position,
        force_handler: shimmer_forces(),
    });

    //let attributes = ParticleAttributes {
    //color: Color::from_rgba(0, 255, 255, 255),
    //friction_coefficient: 0.005,
    //diameter: 5.5,
    //elasticity: 1.,
    //mass: 3.8,
    //animation_options: None,
    //};

    //grid.fill(&attributes, 500, FillStyle::WhiteNoise);

    let texture = load_texture("assets/bubble.png").await.unwrap();
    let attributes = ParticleAttributes {
        color: Color::from_rgba(255, 255, 255, 255),
        texture: Some(texture),
        friction_coefficient: 0.001,
        diameter: 6.,
        elasticity: 1.,
        mass: 2.0,
        animation_options: Some(shimmer_animations()),
        trail_handler: None,
    };

    grid.fill(&attributes, 50, FillStyle::WhiteNoise);

    let attributes = ParticleAttributes {
        //color: Color::from_rgba(231, 196, 150, 255),
        color: Color::from_rgba(0, 255, 0, 255),
        texture: None,
        friction_coefficient: 0.008,
        diameter: 7.,
        elasticity: 1.,
        mass: 3.,
        trail_handler: Some(trail_animation()),
        animation_options: None,
    };

    grid.fill(&attributes, 100, FillStyle::WhiteNoise);

    grid.add_emitter(smoke());
    //grid.add_emitter(another_emitter());

    //let color = Color::from_rgba(0, 26, 51, 255);
    loop {
        clear_background(BLACK);

        grid.draw();
        grid.draw_ui();

        next_frame().await
    }
}
