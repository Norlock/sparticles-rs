#![allow(dead_code)]

mod accelerating_force;
mod animation;
mod animation_handler;
mod animator;
mod collision;
mod color_animation;
mod constant_force;
mod container;
mod emitter;
mod fill_style;
mod force;
mod force_handler;
mod forcer;
mod gravitational_force;
mod grid;
mod particle;
mod pattern;
mod point;
mod position;
mod size_animation;
mod swarm_emitter;

use grid::{Grid, GridOptions};

use fill_style::FillStyle;
use force::Force;
use macroquad::prelude::*;
use particle::ParticleAttributes;
use pattern::{another_emitter, shimmer_animations, shimmer_forces, smoke};
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

    let attributes = ParticleAttributes {
        color: Color::from_rgba(255, 255, 255, 255),
        friction_coefficient: 0.001,
        diameter: 6.,
        elasticity: 1.,
        mass: 2.0,
        //animation_options: Some(shimmer_animations()),
        animation_options: None,
    };

    grid.fill(&attributes, 50, FillStyle::WhiteNoise);

    let attributes = ParticleAttributes {
        color: Color::from_rgba(20, 255, 0, 255),
        friction_coefficient: 0.008,
        diameter: 7.,
        elasticity: 1.,
        mass: 3.,
        animation_options: None,
    };

    grid.fill(&attributes, 100, FillStyle::WhiteNoise);

    //grid.add_emitter(smoke());
    grid.add_emitter(another_emitter());

    loop {
        clear_background(BLACK);

        grid.draw_ui();
        grid.draw();

        next_frame().await
    }
}
