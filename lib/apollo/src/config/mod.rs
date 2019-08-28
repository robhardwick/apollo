pub mod choice;
pub mod error;
pub mod phrase;
pub mod preset;
pub mod scale;
pub mod synth;
pub mod range;
pub mod rhythm;
pub mod track;

use serde::Deserialize;
use rand::rngs::SmallRng;
use rand::seq::IteratorRandom;

use error::ConfigError;
use preset::ConfigPreset;
use scale::ConfigScale;

/// The configuration for song generation
#[derive(Debug, Deserialize)]
pub struct Config {
    /// A default seed for the random number generator
    pub seed: u64,

    /// A list of song configuration presets
    pub presets: Vec<ConfigPreset>,

    /// A list of musical scales references by song presets
    pub scales: Vec<ConfigScale>,
}

impl Config {
    /// Find the [ConfigPreset](struct.ConfigPreset.html) associated with the specified preset ID.
    /// Returns an [UnknownPreset](enum.ConfigError.html#variant.UnknownPreset) error on failure.
    pub fn preset(&self, id: String) -> Result<&ConfigPreset, ConfigError> {
        self.presets.iter().find(|preset| preset.id == id).ok_or(ConfigError::UnknownPreset(id))
    }

    /// Randomly select a [ConfigScale](struct.ConfigScale.html) from the specified
    /// [ConfigPreset](struct.ConfigPreset.html). This first randomly chooses a tag from the preset's
    /// [scale](struct.ConfigPreset.html#structfield.scale) and then randomly chooses a scale from
    /// the set of all scales that have the tag.
    pub fn scale(&self, preset: &ConfigPreset, rng: &mut SmallRng) -> Result<&ConfigScale, ConfigError> {
        let tag = preset.scale.get(rng).ok_or(ConfigError::NoScaleTag)?;
        let scale = self.scales.iter()
            .filter(|scale| scale.tags.iter().any(|t| t == tag))
            .choose(rng)
            .ok_or(ConfigError::InvalidScaleTag(tag.clone()))?;
        Ok(scale)
    }
}