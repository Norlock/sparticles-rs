use std::time::Duration;
use std::time::Instant;

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

impl Grid {
    pub fn new(
        cell_x_count: usize,
        cell_y_count: usize,
        possibility_x_count: usize,
        possibility_y_count: usize,
        possibility_side_length: usize,
        position: Position,
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

    fn cell_x_index(&self, x_coord: f32) -> usize {
        x_coord as usize / self.cell_width
    }

    fn cell_y_index(&self, y_coord: f32) -> usize {
        y_coord as usize / self.cell_height
    }

    fn possibility_x_index(&self, x_coord: f32) -> usize {
        let x_residual = x_coord as usize % self.cell_width;
        x_residual / self.possibility_side_length
    }

    fn possibility_y_index(&self, y_coord: f32) -> usize {
        let y_residual = y_coord as usize % self.cell_height;
        y_residual / self.possibility_side_length
    }

    fn possibility_index(&self, x_index: usize, y_index: usize) -> usize {
        self.possibility_x_count * y_index + x_index
    }
    /**
     * returns true if index needs to be incremented
     */
    pub fn handle_particle(&mut self, vec_index: usize, spot_index: usize) {
        if self.possibility_spots[vec_index][spot_index].queue_frame == self.frame {
            return;
        }

        let mut particle = self.possibility_spots[vec_index].swap_remove(spot_index);

        let mut transform = Transform::new(&particle);

        let x_out_of_bounds = transform.new_x() < 0. || self.width <= transform.new_x();
        let y_out_of_bounds = transform.new_y() < 0. || self.height <= transform.new_y();
        let elasticity_force = -1. * particle.elasticity_fraction;

        if x_out_of_bounds {
            transform.set_new_vx(transform.vx() * elasticity_force);
        }

        if y_out_of_bounds {
            transform.set_new_vy(transform.vy() * elasticity_force);
        }

        let new_x_spot = self.possibility_x_index(transform.new_x());
        let new_y_spot = self.possibility_y_index(transform.new_y());

        let new_vec_index = self.possibility_index(new_x_spot, new_y_spot);

        // todo get more list if element is hovering over multiple spots.
        for other in self.possibility_spots[new_vec_index].iter_mut() {
            // TODO modify other vx/vy.
            // Store energy instead of direct velocity.
            particle.handle_collision(other, &mut transform);
        }

        particle.update(&self.position, transform);

        if vec_index != new_vec_index {
            particle.queue_frame = self.frame;
        }

        self.possibility_spots[new_vec_index].push(particle);
    }

    pub fn fill(&mut self, attributes: &ParticleAttributes, count: u32, fill_style: FillStyle) {
        self.particle_count = self.particle_count + count;

        match fill_style {
            FillStyle::WhiteNoise => self.fill_white_noise(attributes, count),
        }
    }

    pub fn draw(&mut self) {
        let start = Instant::now();

        for vec_index in 0..self.possibility_spots.len() {
            for index in (0..self.possibility_spots[vec_index].len()).rev() {
                self.handle_particle(vec_index, index);
            }
        }

        if self.frame % 50 == 0 {
            self.duration = start.elapsed();
            //self.debug();
        }

        self.frame += 1;
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

        //for x_index in 0..self.possibility_x_count * self.cell_x_count {
        //for y_index in 0..self.possibility_y_count * self.cell_y_count {
        //let x = self.position.x + (x_index * self.possibility_side_length) as f32;
        //let y = self.position.y + (y_index * self.possibility_side_length) as f32;

        //draw_rectangle_lines(
        //x,
        //y,
        //self.possibility_side_length as f32,
        //self.possibility_side_length as f32,
        //0.3,
        //LIGHTGRAY,
        //);
        //}
        //}
    }

    fn possibility_taken(&self, x_coord: f32, y_coord: f32) -> bool {
        let cell_x_index = self.cell_x_index(x_coord);
        let cell_y_index = self.cell_y_index(y_coord);

        let poss_x_index = self.possibility_x_index(x_coord);
        let poss_y_index = self.possibility_y_index(y_coord);

        self.possibility_spots[self.possibility_index(poss_x_index, poss_y_index)]
            .iter()
            .any(|p| {
                self.cell_x_index(p.x) == cell_x_index && self.cell_y_index(p.y) == cell_y_index
            })
    }

    fn fill_white_noise(&mut self, attributes: &ParticleAttributes, count: u32) {
        let mut i: u32 = 0;
        while i < count {
            let x_coord = rand::gen_range(0., self.width);
            let y_coord = rand::gen_range(0., self.height);
            if !self.possibility_taken(x_coord, y_coord) {
                self.add_particle(x_coord, y_coord, &attributes);
                i = i + 1;
            }
        }
    }

    fn add_particle(&mut self, x_coord: f32, y_coord: f32, attributes: &ParticleAttributes) {
        let particle = Particle::new(x_coord, y_coord, attributes);
        let poss_x_index = self.possibility_x_index(x_coord);
        let poss_y_index = self.possibility_y_index(y_coord);
        let poss_index = self.possibility_index(poss_x_index, poss_y_index);
        self.possibility_spots[poss_index].push(particle);
    }

    pub fn start(&mut self) {}

    pub fn stop(&mut self) {}
}

#[test]
fn create_grid() {
    let grid = Grid::new(5, 5, 10, 10, 10, Position::new(1., 2.));

    assert_eq!(grid.cell_width, 100); // 10 * 10
    assert_eq!(grid.cell_height, 100);
    assert_eq!(grid.possibility_spots.len(), 100); // 10 * 10
    assert_eq!(grid.width, 500.);
    assert_eq!(grid.height, 500.);
    assert_eq!(grid.position.x, 1.);
    assert_eq!(grid.position.y, 2.);
}

#[test]
fn fill_grid() {
    let mut grid = Grid::new(5, 5, 10, 10, 10, Position::new(1., 2.));
    let attributes = ParticleAttributes {
        color: Color::from_rgba(20, 20, 200, 255),
        decay_fraction: 0.5,
        diameter: 5.,
        elasticity_fraction: 0.9,
        weight: 1.,
    };

    grid.fill(&attributes, 200, FillStyle::WhiteNoise);

    assert_eq!(grid.particle_count, 200);

    let len = grid
        .possibility_spots
        .iter()
        .fold(0, |acc, x| acc + x.len()) as u32;

    assert_eq!(grid.particle_count, len);
}

#[test]
fn updates_frame() {
    let mut grid = Grid::new(5, 5, 10, 10, 10, Position::new(1., 2.));
    assert_eq!(0, grid.frame);
    grid.draw();
    assert_eq!(1, grid.frame);
}

#[test]
fn add_particle() {
    let mut grid = Grid::new(5, 5, 10, 10, 10, Position::new(1., 2.));

    let attributes = ParticleAttributes {
        color: Color::from_rgba(20, 20, 200, 255),
        decay_fraction: 0.5,
        diameter: 5.,
        elasticity_fraction: 0.9,
        weight: 1.,
    };

    grid.add_particle(115., 105., &attributes);
    assert_eq!(1, grid.possibility_spots[1].len());

    // if y is 1 more, then the pos in array is + poss_x_count (10).
    grid.add_particle(105., 115., &attributes);
    assert_eq!(1, grid.possibility_spots[10].len());

    let particle = &grid.possibility_spots[1][0];

    assert_eq!(1, grid.possibility_x_index(particle.x));
    assert_eq!(0, grid.possibility_y_index(particle.y));

    assert_eq!(1, grid.cell_x_index(particle.x));
    assert_eq!(1, grid.cell_y_index(particle.y));

    // colors is a number between 0 - 1, (255 / 255).
    assert_eq!(1., particle.color.a);

    assert_eq!(5., particle.diameter);
    assert_eq!(2.5, particle.radius);
    assert_eq!(0.5, particle.decay_fraction);
    assert_eq!(0.9, particle.elasticity_fraction);
    assert_eq!(1., particle.weight);
}

#[test]
fn moves_particle() {
    let mut grid = Grid::new(5, 5, 10, 10, 10, Position::new(1., 2.));

    let attributes = ParticleAttributes {
        color: Color::from_rgba(20, 20, 200, 255),
        decay_fraction: 0.5,
        diameter: 5.,
        elasticity_fraction: 0.9,
        weight: 1.,
    };

    grid.add_particle(5., 5., &attributes);

    assert_eq!(1, grid.possibility_spots[0].len());

    //grid.possibility_spots[0][0].vx += 1.;
    //grid.draw();

    //assert_eq!(1, grid.possibility_spots[1].len());
}
