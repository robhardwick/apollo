pub mod adsr;

use serde::Deserialize;

use adsr::ConfigADSR;
use crate::config::range::Range;

#[derive(Clone, Debug, Deserialize)]
pub struct ConfigNote {
    pub frequency: Range<f32>,
    pub adsr: ConfigADSR,
}