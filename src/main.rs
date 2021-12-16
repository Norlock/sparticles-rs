mod animation;
mod collision;
mod container;
mod fill_style;
mod force;
mod force_builder;
mod grid;
mod particle;
mod position;

use animation::AnimationData;
use force_builder::ForceBuilder;
use grid::{Grid, GridOptions};

use fill_style::FillStyle;
use force::{Force, ForceType};
use macroquad::prelude::*;
use particle::{InitFrame, ParticleAttributes};
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
        forces: forces(),
    });

    let attributes = ParticleAttributes {
        color: Color::from_rgba(20, 200, 200, 255),
        friction: 1.,
        diameter: 5.2,
        elasticity_fraction: 0.98,
        mass: 1.,
        init_frame: InitFrame::Random,
    };

    grid.fill(&attributes, 200, FillStyle::WhiteNoise);

    let attributes = ParticleAttributes {
        color: Color::from_rgba(20, 200, 100, 255),
        friction: 1.,
        diameter: 6.5,
        elasticity_fraction: 0.98,
        mass: 1.5,
        init_frame: InitFrame::Random,
    };

    grid.fill(&attributes, 200, FillStyle::WhiteNoise);

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

    builder.build()
}

fn animate(data: &mut AnimationData, frame: u32) {
    //if frame % 50 == 0 {
    //data.color.r = rand::gen_range(0., 1.);
    //data.color.g = rand::gen_range(0., 1.);
    //data.color.b = rand::gen_range(0., 1.);
    //}

    //if frame % 20 == 0 {
    //data.color.a = (frame as f32 / 50.).sin().abs();
    //}
}
