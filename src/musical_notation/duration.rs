/**
 * Defines the duration of a MusicalElement using the
 * [time unit box system](https://en.wikipedia.org/wiki/Time_unit_box_system).
 * The number that Duration contains refers the the number of boxes of a fixed unit of time
 * that the MusicalElement is played for.
 */
#[derive(Debug, Copy, Clone)]
pub struct Duration(pub u16);

impl Duration {
    pub fn get_time_units(&self) -> u16 {
        self.0
    }
}
