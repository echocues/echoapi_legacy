use rocket::serde::{Serialize, Deserialize};

use super::time::EventTime;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Event {
    pub time: EventTime,
    pub cues: Vec<String>,
    pub notes: Vec<String>,
}
