use serde::{Serialize, Deserialize};
use super::{soundcue::SoundCue, scene::Scene};

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub project_id: String,
    pub title: String,
    pub description: String,
    pub sound_cues: Vec<SoundCue>,
    pub scenes: Vec<Scene>,
}
