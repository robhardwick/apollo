pub mod adsr;

use serde::Deserialize;

use adsr::ConfigADSR;
use crate::config::range::ConfigRange;

/// A musical note configuration
#[derive(Clone, Debug, Deserialize)]
pub struct ConfigNote {
    pub frequency: ConfigRange<f32>,
    pub adsr: ConfigADSR,
}