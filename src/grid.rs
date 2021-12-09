use std::time::Instant;
use std::{
    borrow::{Borrow, BorrowMut},
    time::Duration,
};

use crate::particle;
use crate::{
    fill_style::FillStyle,
    particle::{Particle, ParticleAttributes},
    position::Position,
    transform::Transform,
};
use macroquad::prelude::*;

#[derive(Debug)]
pub struct Grid {
    pub possibility_spots: Vec<Vec<Particle>>,
    pub cell_x_count: usize,
    pub cell_y_count: usize,
    pub possibility_x_count: usize,
    pub possibility_y_count: usize,
    pub possibility_side_length: usize,
    pub position: Position,
    pub show_ui: bool,
    pub frame: u32,
    pub width: f32,
    pub height: f32,
    pub cell_width: usize,
    pub cell_height: usize,
    pub duration: Duration,
    pub particle_count: u32,
}

fn create_possibility_grid(
    possibility_x_count: usize,
    possiblity_y_count: usize,
) -> Vec<Vec<Particle>> {
    let mut spots: Vec<Vec<Particle>> = Vec::new();

    for _ in 0..possibility_x_count {
        for _ in 0..possiblity_y_count {
            spots.push(Vec::new());
        }
    }

    return spots;
}

fn particle_actions(position: &Position, particle: &mut Particle, transform: Transform) {
    //particle.handle_collision(self);
    particle.transform(transform);
    particle.draw(&position);
}

impl Grid {
    pub fn new(
        cell_x_count: usize,
        cell_y_count: usize,
        possibility_x_count: usize,
        possibility_y_count: usize,
        possibility_side_length: usize,
        position: Position,
        show_ui: bool,
    ) -> Self {
        let cell_width = possibility_x_count * possibility_side_length;
        let cell_height = possibility_y_count * possibility_side_length;
        let width = (cell_x_count * cell_width) as f32;
        let height = (cell_y_count * cell_height) as f32;
        let possibility_spots = create_possibility_grid(possibility_x_count, possibility_y_count);

        Self {
            cell_x_count,
            cell_y_count,
            possibility_x_count,
            possibility_y_count,
            possibility_side_length,
            position,
            show_ui,
            width,
            height,
            possibility_spots,
            cell_width,
            cell_height,
            frame: 0,
            duration: Duration::new(0, 0),
            particle_count: 0,
        }
    }

    pub fn debug(&self) {
        for v_index in 0..self.possibility_spots.len() {
            println!(
                "possiblity x: {}, y: {}, has {} particles.",
                v_index % self.possibility_x_count,
                v_index / self.possibility_x_count,
                self.possibility_spots[v_index].len()
            );
        }
        println!("------------------");
    }

    pub fn cell_x_index(&self, x_coord: f32) -> usize {
        x_coord as usize / self.cell_width
    }

    pub fn cell_y_index(&self, y_coord: f32) -> usize {
        y_coord as usize / self.cell_height
    }

    pub fn possibility_x_index(&self, x_coord: f32) -> usize {
        let x_residual = x_coord as usize % self.cell_width;
        x_residual / self.possibility_side_length
    }

    pub fn possibility_y_index(&self, y_coord: f32) -> usize {
        let y_residual = y_coord as usize % self.cell_height;
        y_residual / self.possibility_side_length
    }

    pub fn handle_particle(&mut self, vec_index: usize, spot_index: usize) -> bool {
        let particle = self.possibility_spots[vec_index][spot_index].borrow();
        if particle.queue_frame == self.frame {
            return false;
        }

        let mut transform = Transform::new(&particle);

        let new_x = particle.x + transform.vx;
        let new_y = particle.y + transform.vy;

        let x_out_of_bounds = new_x < 0. || self.width <= new_x;
        let y_out_of_bounds = new_y < 0. || self.height <= new_y;

        if x_out_of_bounds {
            transform.vx = transform.vx * -1.;
        }

        if y_out_of_bounds {
            transform.vy = transform.vy * -1.;
        }

        let new_x_spot = self.possibility_x_index(particle.x + transform.vx);
        let new_y_spot = self.possibility_y_index(particle.y + transform.vy);
        let new_vec_index = new_y_spot * self.possibility_x_count + new_x_spot;

        if vec_index != new_vec_index {
            let mut m_particle = self.possibility_spots[vec_index].remove(spot_index);
            m_particle.queue_frame = self.frame;
            particle_actions(&self.position, &mut m_particle, transform);
            self.possibility_spots[new_y_spot * self.possibility_x_count + new_x_spot]
                .push(m_particle);
            return true;
        } else {
            let mut m_particle = self.possibility_spots[vec_index][spot_index].borrow_mut();
            particle_actions(&self.position, &mut m_particle, transform);
            return false;
        }
    }

    pub fn fill(&mut self, attributes: &ParticleAttributes, count: u32, fill_style: FillStyle) {
        self.particle_count = self.particle_count + count;

        match fill_style {
            FillStyle::BlueNoise => self.fill_blue_noise(attributes, count),
            FillStyle::WhiteNoise => self.fill_white_noise(attributes, count),
        }
    }

    pub fn draw(&mut self) {
        let start = Instant::now();

        for vec_index in 0..self.possibility_spots.len() {
            let mut spot_len = self.possibility_spots[vec_index].len();
            let mut spot_index = 0;

            while spot_index < spot_len {
                if self.handle_particle(vec_index, spot_index) {
                    spot_len = spot_len - 1;
                } else {
                    spot_index = spot_index + 1;
                }
            }
        }

        if self.frame % 50 == 0 {
            self.duration = start.elapsed();
            //self.debug();
        }

        self.frame = self.frame + 1;
    }

    pub fn draw_ui(&self) {
        draw_text(
            format!("particle count: {}", self.particle_count).as_str(),
            10.0,
            20.0,
            20.0,
            WHITE,
        );

        draw_text(
            format!("Loop time: {:?}", self.duration).as_str(),
            10.0,
            40.0,
            20.0,
            WHITE,
        );
        //draw_rectangle_lines(5., 5., 10., 10., 2., GREEN);
    }

    fn possibility_taken(&self, x_coord: f32, y_coord: f32) -> bool {
        let cell_x_index = self.cell_x_index(x_coord);
        let cell_y_index = self.cell_y_index(y_coord);

        let poss_x_index = self.possibility_x_index(x_coord);
        let poss_y_index = self.possibility_y_index(y_coord);

        self.possibility_spots[poss_y_index * self.possibility_x_count + poss_x_index]
            .iter()
            .any(|p| {
                self.cell_x_index(p.x) == cell_x_index && self.cell_y_index(p.y) == cell_y_index
            })
    }

    fn fill_blue_noise(&mut self, attributes: &ParticleAttributes, count: u32) {}

    fn fill_white_noise(&mut self, attributes: &ParticleAttributes, count: u32) {
        //fn fill_cell(x_coord: u16, y_coord: u16) {}

        let mut i = 0;
        while i < count {
            let x_coord = rand::gen_range(0., self.width);
            let y_coord = rand::gen_range(0., self.height);
            if !self.possibility_taken(x_coord as f32, y_coord as f32) {
                self.add_particle(x_coord as f32, y_coord as f32, &attributes);
                i = i + 1;
            }
        }
    }

    fn add_particle(&mut self, x_coord: f32, y_coord: f32, attributes: &ParticleAttributes) {
        let particle = Particle::new(x_coord, y_coord, attributes);
        let possibility_x_index = self.possibility_x_index(x_coord);
        let possibility_y_index = self.possibility_y_index(y_coord);
        self.possibility_spots
            [possibility_y_index * self.possibility_x_count + possibility_x_index]
            .push(particle);
    }

    pub fn start(&mut self) {}

    pub fn stop(&mut self) {}
}
