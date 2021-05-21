use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes: i32;
        let mut clock_hours: i32;

        if hours < 0 && hours > -24 {
            clock_hours = 24 + hours;
        } else if hours < -23 {
            clock_hours = 24 + (hours % 24);
            println!(
                "Inside logic. clock_hours: {}, hours: {}",
                clock_hours, hours
            );
        } else {
            clock_hours = hours;
        }

        total_minutes = (clock_hours * 60) + minutes;

        println!(
            "total_minutes: {}, clock_hours: {}",
            total_minutes, clock_hours
        );

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
