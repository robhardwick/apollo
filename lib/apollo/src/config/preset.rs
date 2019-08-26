use serde::Deserialize;

use crate::config::choice::Choice;
use crate::config::rhythm::ConfigRhythm;
use crate::config::track::ConfigTrack;

#[derive(Clone, Debug, Deserialize)]
pub struct ConfigPreset {
    pub id: String,
    pub rhythm: ConfigRhythm,
    pub scale: Choice<String>,
    pub tracks: Vec<ConfigTrack>,
}