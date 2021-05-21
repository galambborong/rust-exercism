use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // let total_minutes: i32 = (hours * 60) + minutes;
        let mut clock_hours = hours;
        let mut clock_minutes = minutes;
        let mut mins_into_hours = minutes;

        if hours > 23 {
            clock_hours = hours % 24;
        }

        if minutes > 59 {
            mins_into_hours = minutes / 60;
            clock_hours += mins_into_hours;
            clock_minutes -= 60;
        }

        Clock {
            hours: clock_hours,
            minutes: clock_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
