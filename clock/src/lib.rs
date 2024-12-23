use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}


impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;
        // Normalize total minutes to one day (1440 minutes in a day)
        let normalized_minutes = (total_minutes % 1440 + 1440) % 1440;
        let hours = normalized_minutes / 60;
        let minutes = normalized_minutes % 60;

        Self { hours, minutes }
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        let minutes = self.minutes + minutes;

        Self::new(self.hours, minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // Format the clock as HH:MM
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}
