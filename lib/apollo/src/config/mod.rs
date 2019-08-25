pub mod phrase;
pub mod preset;
pub mod synth;
pub mod track;

mod error;
mod range;

use serde::Deserialize;

use error::ConfigError;
use preset::ConfigPreset;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub seed: u64,
    pub presets: Vec<ConfigPreset>,
}

impl Config {
    pub fn preset(&self, id: String) -> Result<&ConfigPreset, ConfigError> {
        self.presets.iter().find(|preset| preset.id == id).ok_or(ConfigError::UnknownPreset(id))
    }
}