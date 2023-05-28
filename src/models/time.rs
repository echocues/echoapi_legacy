use std::fmt::Display;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct EventTime {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
}

impl Display for EventTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.hours, self.minutes, self.seconds)
    }
}
