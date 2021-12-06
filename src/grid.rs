use crate::{
    fill_style::FillStyle,
    particle::{Particle, ParticleAttributes},
    position::Position,
};
use macroquad::prelude::*;

#[derive(Debug)]
pub struct Grid {
    pub cell_x_count: u16,
    pub cell_y_count: u16,
    pub possibility_x_count: u16,
    pub possibility_y_count: u16,
    pub possibility_spots: Vec<Vec<Particle>>,
    pub possibility_side_length: u16,
    pub position: Position,
    pub show_ui: bool,
    width: u16,
    height: u16,
}

fn create_possibility_grid(cell_x_count: u16, cell_y_count: u16) -> Vec<Vec<Particle>> {
    let mut spots: Vec<Vec<Particle>> = Vec::new();

    for _ in 0..cell_x_count {
        for _ in 0..cell_y_count {
            spots.push(Vec::new());
        }
    }

    return spots;
}

impl Grid {
    pub fn new(
        cell_x_count: u16,
        cell_y_count: u16,
        possibility_x_count: u16,
        possibility_y_count: u16,
        possibility_side_length: u16,
        position: Position,
        show_ui: bool,
    ) -> Self {
        let width = cell_x_count * possibility_x_count * possibility_side_length;
        let height = cell_y_count * possibility_y_count * possibility_side_length;
        let possibility_spots = create_possibility_grid(cell_x_count, cell_y_count);

        Self {
            cell_x_count,
            cell_y_count,
            possibility_x_count,
            possibility_y_count,
            possibility_side_length,
            position,
            possibility_spots,
            width,
            height,
            show_ui,
        }
    }

    pub fn fill(&mut self, attributes: ParticleAttributes, count: u32, fill_style: FillStyle) {
        match fill_style {
            FillStyle::BlueNoise => self.fill_blue_noise(&attributes, count),
            FillStyle::WhiteNoise => self.fill_white_noise(attributes, count),
        }
    }

    pub fn cell_x_index(&self, x_coord: f32) -> u16 {
        x_coord as u16 / self.cell_x_count
    }

    pub fn cell_y_index(&self, y_coord: f32) -> u16 {
        y_coord as u16 / self.cell_y_count
    }

    pub fn draw(&self) {
        for vec in &self.possibility_spots {
            for particle in vec {
                particle.draw();
            }
        }

        draw_rectangle_lines(5., 5., 10., 10., 2., GREEN);
        draw_text(
            format!("particle count: {}", 100).as_str(),
            10.0,
            20.0,
            20.0,
            WHITE,
        );
    }

    fn possibility_taken(&self, x_coord: f32, y_coord: f32) -> bool {
        let cell_x_index = self.cell_x_index(x_coord);
        let cell_y_index = self.cell_y_index(y_coord);

        let poss_x_index = self.possibility_x_index(x_coord);
        let poss_y_index = self.possibility_y_index(y_coord);

        self.possibility_spots[(poss_x_index * poss_y_index) as usize]
            .iter()
            .any(|p| p.cell_x_index == cell_x_index && p.cell_y_index == cell_y_index)
    }

    fn possibility_x_index(&self, x_coord: f32) -> u16 {
        x_coord as u16 % (self.possibility_x_count / self.possibility_side_length)
    }

    fn possibility_y_index(&self, y_coord: f32) -> u16 {
        y_coord as u16 % (self.possibility_y_count / self.possibility_side_length)
    }

    fn fill_blue_noise(&mut self, attributes: &ParticleAttributes, count: u32) {}

    fn fill_white_noise(&mut self, attributes: ParticleAttributes, count: u32) {
        //fn fill_cell(x_coord: u16, y_coord: u16) {}

        let mut i = 0;
        while i < count {
            let x_coord = rand::gen_range(0, self.width as u32);
            let y_coord = rand::gen_range(0, self.height as u32);
            if !self.possibility_taken(x_coord as f32, y_coord as f32) {
                self.add_particle(x_coord as f32, y_coord as f32, &attributes);
                i = i + 1;
            }
        }
    }

    pub fn get_possibility_spot(&mut self, x_coord: f32, y_coord: f32) -> &mut Vec<Particle> {
        let possibility_x_index = self.possibility_x_index(x_coord);
        let possibility_y_index = self.possibility_y_index(y_coord);
        &mut self.possibility_spots[(possibility_x_index * possibility_y_index) as usize]
    }

    fn add_particle(&mut self, x_coord: f32, y_coord: f32, attributes: &ParticleAttributes) {
        let cell_x_index = self.cell_x_index(x_coord);
        let cell_y_index = self.cell_y_index(y_coord);

        let mut particle = Particle::new(x_coord, y_coord, cell_x_index, cell_y_index, attributes);
        if x_coord < 50. {
            particle.color.b = 10.;
        }

        self.get_possibility_spot(x_coord, y_coord).push(particle);
        // set on grid
    }

    pub fn start(&mut self) {}

    pub fn stop(&mut self) {}
}
