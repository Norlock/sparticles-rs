mod animation;
mod collision;
mod container;
mod fill_style;
mod force;
mod grid;
mod particle;
mod position;

use crate::animation::AnimationData;
use crate::grid::GridOptions;
use std::rc::Rc;

use crate::grid::Grid;
use fill_style::FillStyle;
use force::{Force, ForceType};
use macroquad::prelude::*;
use particle::{InitFrame, ParticleAttributes};
use position::Position;

#[macroquad::main("Particles")]
async fn main() {
    let position = Position::new(100., 100.);
    let mut forces: Vec<Force> = Vec::new();

    forces.push(Force {
        frames: 50,
        force_type: ForceType::Static { vx: 0.02, vy: 0.01 },
    });

    forces.push(Force {
        frames: 100,
        force_type: ForceType::None,
    });

    forces.push(Force {
        frames: 50,
        force_type: ForceType::Static {
            vx: -0.02,
            vy: -0.015,
        },
    });

    let mut grid = Grid::new(GridOptions {
        cell_x_count: 10,
        cell_y_count: 10,
        possibility_x_count: 10,
        possibility_y_count: 10,
        possibility_side_length: 10,
        position,
        forces,
    });

    let attributes = ParticleAttributes {
        color: Color::from_rgba(20, 200, 200, 255),
        friction: 1.,
        diameter: 5.2,
        elasticity_fraction: 0.98,
        mass: 1.,
        animation: Rc::new(animate),
        last_frame: 100000,
        init_frame: InitFrame::Random,
    };

    grid.fill(&attributes, 200, FillStyle::WhiteNoise);

    let attributes = ParticleAttributes {
        color: Color::from_rgba(20, 200, 100, 255),
        friction: 1.,
        diameter: 6.,
        elasticity_fraction: 0.98,
        mass: 1.5,
        animation: Rc::new(animate),
        last_frame: 100000,
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
