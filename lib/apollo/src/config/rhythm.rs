use serde::Deserialize;

use crate::config::choice::ConfigChoice;
use crate::config::range::ConfigRange;

/// A musical rhythm configuration
#[derive(Clone, Debug, Deserialize)]
pub struct ConfigRhythm {
    /// A range of bpm (beats per minute) values
    pub bpm: ConfigRange<u8>,

    /// A range of beats per bar values (time signature upper numeral)
    pub beat: ConfigRange<u8>,

    /// A list of beat units from which one will be randomly selected (time signature lower numeral)
    pub unit: ConfigChoice<u8>,

    /// The note length weighting configuration for this rhythm
    pub weight: ConfigRhythmWeight,
}

/// A note length weighting configuration for this rhythm
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfigRhythmWeight {
    /// Increase weighting of shorter notes
    Shorter,

    /// Increase weighting of longer notes
    Longer,
}