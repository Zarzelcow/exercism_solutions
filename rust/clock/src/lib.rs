use std::fmt::Formatter;
use time::{Duration, Time};

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    time: Time,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut t = Time::from_hms(0,0,0).unwrap();
        t += Duration::hours(hours as i64);
        t += Duration::minutes(minutes as i64);
        return Clock { time: t };
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self {
            time: self.time + Duration::minutes(minutes as i64)
        }
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.time.hour(), self.time.minute())
    }
}