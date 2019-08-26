pub mod choice;
pub mod phrase;
pub mod preset;
pub mod scale;
pub mod synth;
pub mod rhythm;
pub mod track;

mod error;
mod range;

use serde::Deserialize;
use rand::rngs::SmallRng;
use rand::seq::IteratorRandom;

use error::ConfigError;
use preset::ConfigPreset;
use scale::ConfigScale;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub seed: u64,
    pub presets: Vec<ConfigPreset>,
    pub scales: Vec<ConfigScale>,
}

impl Config {
    pub fn preset(&self, id: String) -> Result<&ConfigPreset, ConfigError> {
        self.presets.iter().find(|preset| preset.id == id).ok_or(ConfigError::UnknownPreset(id))
    }

    pub fn scale(&self, preset: &ConfigPreset, rng: &mut SmallRng) -> Result<&ConfigScale, ConfigError> {
        let tag = preset.scale.random(rng).ok_or(ConfigError::NoScaleTag)?;
        let scale = self.scales.iter()
            .filter(|scale| scale.tags.iter().any(|t| t == tag))
            .choose(rng)
            .ok_or(ConfigError::InvalidScaleTag(tag.clone()))?;
        Ok(scale)
    }
}