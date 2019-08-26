use serde::Deserialize;

use crate::config::choice::Choice;
use crate::config::range::Range;

#[derive(Clone, Debug, Deserialize)]
pub struct ConfigRhythm {
    pub bpm: Range<u8>,
    pub beat: Range<u8>,
    pub unit: Choice<u8>,
    pub weight: ConfigRhythmWeight,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfigRhythmWeight {
    Shorter,
    Longer,
}