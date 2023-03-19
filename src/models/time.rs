use std::fmt::Display;

use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct EventTime {
    pub hour: u32,
    pub minutes: u32,
    pub seconds: u32,
}

impl Display for EventTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.hour, self.minutes, self.seconds)
    }
}
