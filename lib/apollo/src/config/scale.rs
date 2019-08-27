use serde::Deserialize;

/// A musical scale configuration
#[derive(Clone, Debug, Deserialize)]
pub struct ConfigScale {
    /// The name of the musical scale configuration
    pub name: String,

    /// A list of tags referenced by [ConfigPresets](struct.ConfigPreset.html) such as "major" or "diatonic"
    pub tags: Vec<String>,

    /// A list of pitches for the scale specified in Hz with the tonic or tonal centre as the first pitch
    pub notes: Vec<f32>,
}