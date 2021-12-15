//pub type ApplyForce = fn(data: &mut AnimationData, frame: u32);

#[derive(Debug)]
pub enum ForceType {
    Static { vx: f32, vy: f32 },
    Newton { nx: f32, ny: f32 },
    None,
}

#[derive(Debug)]
pub struct Force {
    pub frames: u32,
    pub force_type: ForceType,
}
