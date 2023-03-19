use rocket::serde::{Serialize, Deserialize};

use super::event::Event;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Scene {
    pub name: String,
    pub events: Vec<Event>,
}
