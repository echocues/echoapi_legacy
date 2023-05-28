use serde::{Serialize, Deserialize};
use super::ease_settings::EaseSetting;

#[derive(Serialize, Deserialize)]
pub struct SoundCue {
    pub identifier: String,
    pub file_name: String,
    pub ease_in: EaseSetting,
    pub ease_out: EaseSetting,
    pub volume: f32,
    pub speed: f32,
}
