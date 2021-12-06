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
    let position = Position { x: 50, y: 50 };
    let mut grid = Grid::new(5, 5, 10, 10, 10, position, true);

    let attributes = ParticleAttributes {
        color: Color::from_rgba(20, 20, 200, 255),
        decay: 0.5,
        diameter: 2.5,
    };

    grid.fill(attributes, 100, FillStyle::WhiteNoise);

    loop {
        //clear_background(BLACK);

        grid.draw();

        next_frame().await
    }
}
