use crate::force::{Force, ForceType};

pub struct ForceBuilder {
    forces: Vec<Force>,
}

impl ForceBuilder {
    pub fn new() -> Self {
        Self { forces: Vec::new() }
    }

    pub fn add(&mut self, force_type: ForceType, frames: u32) {
        let previous_last_frame = if let Some(force) = self.forces.last() {
            force.until_frame
        } else {
            0
        };

        self.forces.push(Force {
            until_frame: previous_last_frame + frames,
            force_type,
        })
    }

    pub fn build(self) -> Vec<Force> {
        self.forces
    }
}
