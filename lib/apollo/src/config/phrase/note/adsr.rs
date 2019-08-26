use serde::Deserialize;

use crate::config::range::Range;

#[derive(Clone, Debug, Deserialize)]
pub struct ConfigADSR {
    pub amplitude: Range<f32>,
    pub attack: Range<f32>,
    pub release: Range<f32>,
}