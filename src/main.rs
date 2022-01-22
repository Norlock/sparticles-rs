#![allow(dead_code)]

mod animation;
mod animation_handler;
mod animator;
mod collision;
mod color_animation;
mod container;
mod emitter;
mod fill_style;
mod force;
mod force_handler;
mod forcer;
mod grid;
mod newton_force;
mod particle;
mod pattern;
mod position;
mod size_animation;

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
        cell_x_count: 5,
        cell_y_count: 5,
        possibility_x_count: 10,
        possibility_y_count: 10,
        possibility_side_length: 10,
        position,
        force_handler: shimmer_forces(),
    });

    let attributes = ParticleAttributes {
        color: Color::from_rgba(0, 255, 255, 255),
        friction_coefficient: 0.005,
        diameter: 5.2,
        elasticity: 1.,
        mass: 1.8,
        animation_options: None,
    };

    grid.fill(&attributes, 50, FillStyle::WhiteNoise);

    let attributes = ParticleAttributes {
        color: Color::from_rgba(255, 255, 255, 255),
        friction_coefficient: 0.001,
        diameter: 5.,
        elasticity: 1.,
        mass: 1.5,
        animation_options: Some(shimmer_animations()),
    };

    grid.fill(&attributes, 50, FillStyle::WhiteNoise);

    let attributes = ParticleAttributes {
        color: Color::from_rgba(255, 0, 0, 255),
        friction_coefficient: 0.008,
        diameter: 7.,
        elasticity: 1.,
        mass: 2.5,
        animation_options: None,
    };

    grid.fill(&attributes, 50, FillStyle::WhiteNoise);

    //grid.add_emitter(smoke());
    //grid.add_emitter(another_emitter());

    loop {
        clear_background(BLACK);

        grid.draw_ui();
        grid.draw();

        next_frame().await
    }
}
