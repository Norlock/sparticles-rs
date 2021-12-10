mod container;
mod fill_style;
mod grid;
mod mesh;
mod particle;
mod position;
mod transform;

use crate::grid::Grid;
use fill_style::FillStyle;
use macroquad::prelude::*;
use particle::ParticleAttributes;
use position::Position;

#[macroquad::main("BasicShapes")]
async fn main() {
    let position = Position::new(100., 100.);
    let mut grid = Grid::new(5, 5, 5, 5, 20, position, true);

    let attributes = ParticleAttributes {
        color: Color::from_rgba(20, 20, 200, 255),
        decay_fraction: 0.5,
        diameter: 5.,
        elasticity_fraction: 0.9,
        weight: 1.,
    };

    grid.fill(&attributes, 200, FillStyle::WhiteNoise);
    grid.fill(&attributes, 50, FillStyle::WhiteNoise);

    loop {
        //clear_background(BLACK);

        grid.draw_ui();
        grid.draw();

        next_frame().await
    }
}
