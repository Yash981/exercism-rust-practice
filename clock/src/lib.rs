use std::fmt::{Display, Formatter, Result};
#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = (hours * 60 + minutes).rem_euclid(24 * 60);
        let actual_hours = total_minutes / 60;
        let actual_minutes = total_minutes % 60;
        Clock {
            hours: actual_hours,
            minutes: actual_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
