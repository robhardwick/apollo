use serde::Deserialize;

use crate::config::choice::ConfigChoice;
use crate::config::rhythm::ConfigRhythm;
use crate::config::track::ConfigTrack;

/// A song preset configuration
#[derive(Clone, Debug, Deserialize)]
pub struct ConfigPreset {
    /// The identifier for this song preset
    pub id: String,

    /// The rhythm configuration for this song preset
    pub rhythm: ConfigRhythm,

    /// A list of [ConfigScale](struct.ConfigScale.html) tags from which one will be randomly selected
    pub scale: ConfigChoice<String>,

    /// A list of track configurations for this song preset
    pub tracks: Vec<ConfigTrack>,
}