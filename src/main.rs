mod animation;
mod container;
mod fill_style;
mod grid;
mod particle;
mod position;
mod transform;

use crate::animation::AnimationData;
use std::rc::Rc;

use crate::grid::Grid;
use fill_style::FillStyle;
use macroquad::prelude::*;
use particle::{InitFrame, ParticleAttributes};
use position::Position;

#[macroquad::main("BasicShapes")]
async fn main() {
    let position = Position::new(100., 100.);
    let mut grid = Grid::new(5, 5, 10, 10, 10, position);

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

    fn animate2(data: &mut AnimationData, frame: u32) {
        //if frame % 50 == 0 {
        //data.color.r = rand::gen_range(0., 1.);
        //data.color.g = rand::gen_range(0., 1.);
        //data.color.b = rand::gen_range(0., 1.);
        //}

        //if frame % 20 == 0 {
        //data.color.a = (frame as f32 / 50.).sin().abs();
        //}
    }

    let mut attributes = ParticleAttributes {
        color: Color::from_rgba(20, 20, 200, 255),
        decay_fraction: 0.5,
        diameter: 5.,
        elasticity_fraction: 0.95,
        weight: 1.,
        animation: Rc::new(animate),
        last_frame: 100000,
        init_frame: InitFrame::Random,
    };

    grid.fill(&attributes, 200, FillStyle::WhiteNoise);

    attributes.animation = Rc::new(animate2);

    loop {
        //clear_background(BLACK);

        grid.draw_ui();
        grid.draw();

        next_frame().await
    }
}
