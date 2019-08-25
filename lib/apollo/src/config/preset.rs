use serde::Deserialize;

use crate::config::track::ConfigTrack;

#[derive(Clone, Debug, Deserialize)]
pub struct ConfigPreset {
    pub id: String,
    pub tracks: Vec<ConfigTrack>,
}