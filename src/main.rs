#![allow(dead_code, unused_imports)]

mod animation;
mod animator;
mod collision;
mod container;
mod emitter;
mod fill_style;
mod force;
mod force_builder;
mod grid;
mod particle;
mod pattern;
mod position;

use std::rc::Rc;

use animation::AnimationData;
use animator::Animator;
use force_builder::ForceBuilder;
use grid::{Grid, GridOptions};

use fill_style::FillStyle;
use force::{Force, ForceType};
use macroquad::prelude::*;
use particle::{InitFrame, ParticleAttributes};
use pattern::{another_emitter, shimmer, shimmer_forces, smoke};
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
        forces: shimmer_forces(),
    });

    let attributes = ParticleAttributes {
        color: Color::from_rgba(20, 200, 200, 255),
        friction: 1.,
        diameter: 5.2,
        elasticity_fraction: 0.98,
        mass: 1.,
        init_frame: InitFrame::Random,
        animator: Rc::new(Animator::new(100)),
    };

    grid.fill(&attributes, 200, FillStyle::WhiteNoise);

    let attributes = ParticleAttributes {
        color: Color::from_rgba(255, 255, 0, 255),
        friction: 1.,
        diameter: 5.,
        elasticity_fraction: 0.1,
        mass: 1.5,
        init_frame: InitFrame::Random,
        animator: Rc::new(shimmer()),
    };

    grid.fill(&attributes, 20, FillStyle::WhiteNoise);
    grid.add_emitter(smoke());
    //grid.add_emitter(another_emitter());

    //let attributes = ParticleAttributes {
    //color: Color::from_rgba(200, 20, 20, 255),
    //friction: 1.,
    //diameter: 8.5,
    //elasticity_fraction: 0.98,
    //mass: 2.5,
    //init_frame: InitFrame::Random,
    //animations: Vec::new(),
    //last_frame: 100,
    //};

    //grid.fill(&attributes, 20, FillStyle::WhiteNoise);

    loop {
        clear_background(BLACK);

        grid.draw_ui();
        grid.draw();

        next_frame().await
    }
}

fn forces() -> Vec<Force> {
    let mut builder = ForceBuilder::new();

    builder.add(
        ForceType::Static {
            vx: 0.02,
            vy: 0.015,
        },
        50,
    );

    builder.add(ForceType::None, 100);

    builder.add(
        ForceType::Newton {
            nx: -0.02,
            ny: -0.01,
        },
        50,
    );

    builder.add(
        ForceType::Accelerate {
            vx: -0.1,
            vy: -0.01,
            vx_max: -0.5,
            vy_max: -1.5,
        },
        30,
    );

    builder.add(
        ForceType::Newton {
            nx: -0.022,
            ny: 0.015,
        },
        30,
    );

    builder.build()
}
