use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes: i32 = (hours * 60) + minutes;
        let mut clock_hours = hours;

        if total_minutes / 60 > 23 {
            clock_hours = (total_minutes / 60) % 24
        } else {
            clock_hours = total_minutes / 60
        }

        Clock {
            hours: clock_hours,
            minutes: total_minutes % 60,
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
