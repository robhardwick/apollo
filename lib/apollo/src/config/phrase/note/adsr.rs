use serde::Deserialize;

use crate::config::range::ConfigRange;

/// An attack, decay, sustain, release envelope configuration
#[derive(Clone, Debug, Deserialize)]
pub struct ConfigADSR {
    /// A range of amplitude values from 0.0 to 1.0
    pub amplitude: ConfigRange<f32>,

    /// A range of attack lengths from 0.0 to 1.0
    pub attack: ConfigRange<f32>,

    /// A range of release lengths from 0.0 to 1.0
    pub release: ConfigRange<f32>,
}