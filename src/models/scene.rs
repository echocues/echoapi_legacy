use serde::{Serialize, Deserialize};
use super::event::Event;

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub name: String,
    pub events: Vec<Event>,
}
