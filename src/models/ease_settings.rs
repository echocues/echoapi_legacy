use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct EaseSetting {
    pub enabled: bool,
    pub duration: f32,
}
