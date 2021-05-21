use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes_per_day = 24 * 60;

        let hours_in_24_hr = 24 + (hours % 24);
        let mut total_minutes = (hours_in_24_hr * 60) + minutes;

        while total_minutes < 0 {
            total_minutes += total_minutes_per_day;
        }

        let clock_hours = match total_minutes / 60 {
            24..=i32::MAX => total_minutes / 60 % 24,
            _ => total_minutes / 60,
        };

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
