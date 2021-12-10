use crate::particle::Particle;

pub struct Transform {
    new_x: f32,
    new_y: f32,
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

impl Transform {
    pub fn new(particle: &Particle) -> Self {
        Self {
            new_x: particle.x + particle.vx,
            new_y: particle.y + particle.vy,
            vx: particle.vx,
            vy: particle.vy,
            x: particle.x,
            y: particle.y,
        }
    }

    pub fn set_new_vx(&mut self, vx: f32) {
        self.vx = vx;
        self.new_x = self.x + self.vx;
    }

    pub fn set_new_vy(&mut self, vy: f32) {
        self.vy = vy;
        self.new_y = self.y + self.vy;
    }

    pub fn new_x(&self) -> f32 {
        self.new_x
    }

    pub fn new_y(&self) -> f32 {
        self.new_y
    }

    pub fn vx(&self) -> f32 {
        self.vx
    }

    pub fn vy(&self) -> f32 {
        self.vy
    }
}
