use crate::particle::Particle;

#[derive(Debug)]
pub enum ForceType {
    Static {
        vx: f32,
        vy: f32,
    },
    Newton {
        nx: f32,
        ny: f32,
    },
    Accelerate {
        vx_max: f32,
        vy_max: f32,
        vx: f32,
        vy: f32,
    },
    None,
}

#[derive(Debug)]
pub struct Force {
    pub until_frame: u32,
    pub force_type: ForceType,
}

impl Force {
    pub fn apply(&self, particle: &mut Particle) {
        match self.force_type {
            ForceType::None => (),
            ForceType::Static { vx, vy } => apply_static(particle, vx, vy),
            ForceType::Newton { nx, ny } => apply_newton(particle, nx, ny),
            ForceType::Accelerate {
                vx,
                vy,
                vx_max,
                vy_max,
            } => apply_accelerate(particle, vx, vy, vx_max, vy_max),
        }
    }
}

fn apply_static(particle: &mut Particle, vx: f32, vy: f32) {
    particle.vx += vx;
    particle.vy += vy;
}

fn apply_newton(particle: &mut Particle, nx: f32, ny: f32) {
    particle.vx += nx / particle.mass;
    particle.vy += ny / particle.mass;
}

fn apply_accelerate(particle: &mut Particle, vx: f32, vy: f32, vx_max: f32, vy_max: f32) {
    if 0. <= vx_max && particle.vx <= vx_max || vx_max < 0. && vx_max < particle.vx {
        particle.vx += vx;
    }
    if 0. <= vy_max && particle.vy <= vy_max || vy_max < 0. && vy_max < particle.vy {
        particle.vy += vy;
    }
}
