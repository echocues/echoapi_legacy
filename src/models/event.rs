use serde::{Deserialize, Serialize};
use super::time::EventTime;

#[derive(Serialize, Deserialize)]
pub struct Event {
    pub time: EventTime,
    pub cues: Vec<String>,
    pub notes: Vec<String>,
}
