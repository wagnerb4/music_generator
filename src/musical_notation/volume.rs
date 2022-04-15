#[derive(Debug, Copy, Clone)]
pub struct Volume(u8);

impl Volume {
    pub fn get(&self) -> u8 {
        self.0
    }
}

const STEP_SIZE: u8 = 28;
pub const SILENT: Volume = Volume(0);
pub const PPP: Volume = Volume(1 * STEP_SIZE);
pub const PP: Volume = Volume(2 * STEP_SIZE);
pub const P: Volume = Volume(3 * STEP_SIZE);
pub const MP: Volume = Volume(4 * STEP_SIZE);
pub const M: Volume = Volume(5 * STEP_SIZE);
pub const MF: Volume = Volume(6 * STEP_SIZE);
pub const F: Volume = Volume(7 * STEP_SIZE);
pub const FF: Volume = Volume(8 * STEP_SIZE);
pub const FFF: Volume = Volume(9 * STEP_SIZE);
