use serde::Deserialize;

use crate::config::choice::ConfigChoice;
use crate::config::range::ConfigRange;

/// A synthesizer configuration
#[derive(Clone, Debug, Deserialize)]
pub struct ConfigSynth {
    /// A range of values for the number of base signals for this synthesizer
    pub num: ConfigRange<u8>,

    /// A range of values that the signals for this synthesizer should be offset by
    pub offset: ConfigRange<f32>,

    /// A list of [ConfigSynthSignal](enum.ConfigSynthSignal.html) types from which one will be randomly selected
    pub signal: ConfigChoice<ConfigSynthSignal>,

    /// A list of [ConfigSynthFilter](enum.ConfigSynthSignal.html) types from which one will be randomly selected
    pub filter: ConfigChoice<ConfigSynthFilter>,
}

/// A synthesizer signal type configuration
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfigSynthSignal {
    /// Sine wave
    Sine,

    /// Saw wave
    Saw,

    /// Square wave
    Square,
}

/// A synthesizer filter type configuration
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ConfigSynthFilter {
    /// 4-Pole
    FourPole {
        frequency: ConfigRange<f32>,
        resonance: ConfigRange<f32>,
    },
}