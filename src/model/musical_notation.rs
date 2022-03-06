pub mod pitch {
    /**
     * Defines the pitch of a note in cents.
     */
    #[derive(Debug)]
    pub struct Pitch(f64);
}

pub mod duration {
    /**
     * Defines the duration of a MusicalElement using the
     * [time unit box system](https://en.wikipedia.org/wiki/Time_unit_box_system).
     * The number that Duration contains refers the the number of boxes of fixed unit of time
     * that the MusicalElement is played for.
     */
    #[derive(Debug)]
    pub struct Duration(u16);
}

pub mod volume {
    #[derive(Debug)]
    pub struct Volume(u8);

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
}

pub enum MusicalElement {
    Rest,
    Note {
        pitch: pitch::Pitch,
        duration: duration::Duration,
        volume: volume::Volume,
    },
}
