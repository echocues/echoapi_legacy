use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EaseSetting {
    pub enabled: bool,
    pub duration: f32,
}
