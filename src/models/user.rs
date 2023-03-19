use rocket::serde::{Serialize, Deserialize};

use crate::session_db::SessionDatabase;

use super::projects::Project;

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub username: String,
    pub password: String,
    pub projects: Vec<String>,
}

impl User {
    pub async fn get_projects(&self) -> Vec<Project> {
        let mut res = Vec::new();

        for p in &self.projects {
            match SessionDatabase::get_project(&p).await {
                Some(val) => res.push(val),
                // todo warning message maybe
                None => continue,
            }
        }

        res
    }
}
