use std::time::Instant;

use crate::{
    collision::CollisionData,
    fill_style::FillStyle,
    force::Force,
    particle::{Particle, ParticleAttributes},
    position::Position,
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
    pub last_frame: u32,
    pub width: f32,
    pub height: f32,
    pub cell_width: usize,
    pub cell_height: usize,
    pub duration: u128,
    pub particle_count: u32,
    pub forces: Vec<Force>,
}

pub struct GridOptions {
    pub cell_x_count: usize,
    pub cell_y_count: usize,
    pub possibility_x_count: usize,
    pub possibility_y_count: usize,
    pub possibility_side_length: usize,
    pub position: Position,
    pub forces: Vec<Force>,
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
    pub fn new(options: GridOptions) -> Self {
        let GridOptions {
            cell_x_count,
            cell_y_count,
            possibility_x_count,
            possibility_y_count,
            possibility_side_length,
            position,
            forces,
        } = options;
        let cell_width = possibility_x_count * possibility_side_length;
        let cell_height = possibility_y_count * possibility_side_length;
        let width = (cell_x_count * cell_width) as f32;
        let height = (cell_y_count * cell_height) as f32;
        let possibility_spots = create_possibility_grid(possibility_x_count, possibility_y_count);

        let last_frame: u32 = if let Some(force) = forces.last() {
            force.last_frame
        } else {
            u32::MAX
        };
        println!("{}", last_frame);

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
            duration: 0,
            particle_count: 0,
            last_frame,
            forces,
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

    fn vec_spot_particle(&self, particle: &Particle) -> usize {
        let new_x_spot = self.possibility_x_index(particle.x);
        let new_y_spot = self.possibility_y_index(particle.y);

        self.possibility_index(new_x_spot, new_y_spot)
    }

    fn has_collision(
        &mut self,
        particle: &mut Particle,
        data: &mut CollisionData,
        vec_index: usize,
    ) -> bool {
        for other in self.possibility_spots[vec_index].iter_mut() {
            if particle.handle_possible_collision(other, data) {
                return true;
            }
        }
        return false;
    }

    fn handle_collision(&mut self, particle: &mut Particle, data: &mut CollisionData) -> usize {
        let CollisionData {
            new_x,
            new_y,
            end_new_x,
            end_new_y,
        } = *data;

        let new_x_spot = self.possibility_x_index(new_x);
        let new_y_spot = self.possibility_y_index(new_y);

        let new_vec_index = self.possibility_index(new_x_spot, new_y_spot);

        if self.has_collision(particle, data, new_vec_index) {
            return self.vec_spot_particle(particle);
        }

        let end_x_spot = self.possibility_x_index(end_new_x);
        let end_y_spot = self.possibility_y_index(end_new_y);

        let has_diff_end_x_spot = end_x_spot != new_x_spot;
        let has_diff_end_y_spot = end_y_spot != new_y_spot;

        if has_diff_end_x_spot {
            let new_vec_index = self.possibility_index(end_x_spot, new_y_spot);

            if self.has_collision(particle, data, new_vec_index) {
                return self.vec_spot_particle(particle);
            }
        }

        if has_diff_end_y_spot {
            let new_vec_index = self.possibility_index(new_x_spot, end_y_spot);

            if self.has_collision(particle, data, new_vec_index) {
                return self.vec_spot_particle(particle);
            }
        }

        if has_diff_end_x_spot && has_diff_end_y_spot {
            let new_vec_index = self.possibility_index(end_x_spot, end_y_spot);

            if self.has_collision(particle, data, new_vec_index) {
                return self.vec_spot_particle(particle);
            }
        }

        new_vec_index
    }

    fn apply_force(&self, particle: &mut Particle) {
        for force in self.forces.iter() {
            if self.frame <= force.last_frame {
                return force.apply(particle);
            }
        }
    }

    /**
     * returns true if index needs to be incremented
     */
    pub fn handle_particle(&mut self, vec_index: usize, spot_index: usize) {
        let mut particle = self.possibility_spots[vec_index].swap_remove(spot_index);

        self.apply_force(&mut particle);

        let new_x = particle.x + particle.vx;
        let new_y = particle.y + particle.vy;
        let end_new_x = new_x + particle.diameter;
        let end_new_y = new_y + particle.diameter;

        let x_out_of_bounds = new_x < 0. || self.width <= end_new_x;
        let y_out_of_bounds = new_y < 0. || self.height <= end_new_y;
        // Inverse direction.
        let elasticity_force = -1. * particle.elasticity_fraction;

        if x_out_of_bounds {
            particle.vx *= elasticity_force;
        }

        if y_out_of_bounds {
            particle.vy *= elasticity_force;
        }

        let mut data = CollisionData {
            new_x,
            new_y,
            end_new_x,
            end_new_y,
        };

        let new_vec_index = self.handle_collision(&mut particle, &mut data);

        particle.update(&self.position, self.width, self.height);

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
            for spot_index in (0..self.possibility_spots[vec_index].len()).rev() {
                let mut particle = &mut self.possibility_spots[vec_index][spot_index];
                if particle.queue_frame == self.frame {
                    particle.queue_frame = u32::MAX;
                    continue;
                }

                self.handle_particle(vec_index, spot_index);
            }
        }

        if self.frame % 50 == 0 {
            self.duration = start.elapsed().as_micros();
        }

        if self.frame == self.last_frame {
            self.frame = 0;
        } else {
            self.frame += 1;
        }
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
            format!("Loop time (micro seconds): {}", self.duration).as_str(),
            10.0,
            40.0,
            20.0,
            WHITE,
        );

        if is_mouse_button_pressed(MouseButton::Left) {
            let (raw_x, raw_y) = mouse_position();
            // x, y on screen is an offset of the the grid position.
            let x = raw_x - self.position.x;
            let y = raw_y - self.position.y;
            let x_index = self.possibility_x_index(x);
            let y_index = self.possibility_y_index(y);
            let vec_index = self.possibility_index(x_index, y_index);

            println!("------------------------------");
            for particle in self.possibility_spots[vec_index].iter() {
                let inside_x = particle.x <= x && x <= particle.x + particle.diameter;
                let inside_y = particle.y <= y && y <= particle.y + particle.diameter;

                // No collision
                if inside_x && inside_y {
                    println!("{:#?}", particle);
                }
            }
        }

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
}

//#[test]
//fn create_grid() {
//let grid = Grid::new(5, 5, 10, 10, 10, Position::new(1., 2.));

//assert_eq!(grid.cell_width, 100); // 10 * 10
//assert_eq!(grid.cell_height, 100);
//assert_eq!(grid.possibility_spots.len(), 100); // 10 * 10
//assert_eq!(grid.width, 500.);
//assert_eq!(grid.height, 500.);
//assert_eq!(grid.position.x, 1.);
//assert_eq!(grid.position.y, 2.);
//}

//#[test]
//fn fill_grid() {
//let mut grid = Grid::new(5, 5, 10, 10, 10, Position::new(1., 2.));
//let attributes = ParticleAttributes {
//color: Color::from_rgba(20, 20, 200, 255),
//friction: 0.5,
//diameter: 5.,
//elasticity_fraction: 0.9,
//mass: 1.,
//};

//grid.fill(&attributes, 200, FillStyle::WhiteNoise);

//assert_eq!(grid.particle_count, 200);

//let len = grid
//.possibility_spots
//.iter()
//.fold(0, |acc, x| acc + x.len()) as u32;

//assert_eq!(grid.particle_count, len);
//}

//#[test]
//fn updates_frame() {
//let mut grid = Grid::new(5, 5, 10, 10, 10, Position::new(1., 2.));
//assert_eq!(0, grid.frame);
//grid.draw();
//assert_eq!(1, grid.frame);
//}

//#[test]
//fn add_particle() {
//let mut grid = Grid::new(5, 5, 10, 10, 10, Position::new(1., 2.));

//let attributes = ParticleAttributes {
//color: Color::from_rgba(20, 20, 200, 255),
//friction: 0.5,
//diameter: 5.,
//elasticity_fraction: 0.9,
//mass: 1.,
//};

//grid.add_particle(115., 105., &attributes);
//assert_eq!(1, grid.possibility_spots[1].len());

//// if y is 1 more, then the pos in array is + poss_x_count (10).
//grid.add_particle(105., 115., &attributes);
//assert_eq!(1, grid.possibility_spots[10].len());

//let particle = &grid.possibility_spots[1][0];

//assert_eq!(1, grid.possibility_x_index(particle.x));
//assert_eq!(0, grid.possibility_y_index(particle.y));

//assert_eq!(1, grid.cell_x_index(particle.x));
//assert_eq!(1, grid.cell_y_index(particle.y));

//// colors is a number between 0 - 1, (255 / 255).
//assert_eq!(1., particle.color.a);

//assert_eq!(5., particle.diameter);
//assert_eq!(2.5, particle.radius);
//assert_eq!(0.5, particle.friction);
//assert_eq!(0.9, particle.elasticity_fraction);
//assert_eq!(1., particle.mass);
//}

//#[test]
//fn moves_particle() {
//let mut grid = Grid::new(5, 5, 10, 10, 10, Position::new(1., 2.));

//let attributes = ParticleAttributes {
//color: Color::from_rgba(20, 20, 200, 255),
//friction: 0.5,
//diameter: 5.,
//elasticity_fraction: 0.9,
//mass: 1.,
//};

//grid.add_particle(5., 5., &attributes);

//assert_eq!(1, grid.possibility_spots[0].len());

////grid.possibility_spots[0][0].vx += 1.;
////grid.draw();

////assert_eq!(1, grid.possibility_spots[1].len());
//}
