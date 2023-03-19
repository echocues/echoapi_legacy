use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct CreateProjectParams {
    pub title: String,
    pub description: String,
    pub username: String,
}
