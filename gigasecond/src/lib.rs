use chrono::{DateTime, Duration, Utc};

/// Return a Utc DateTime one gigasecond after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    start + Duration::seconds(1_000_000_000)
}
