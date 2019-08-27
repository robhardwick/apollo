use serde::Deserialize;

use crate::config::phrase::ConfigPhrase;
use crate::config::synth::ConfigSynth;
use crate::config::range::ConfigRange;

/// A track configuration consisting of a musical phrase and instrument
#[derive(Clone, Debug, Deserialize)]
pub struct ConfigTrack {
    /// A range of values for the number of tracks of this configuration to be generated
    pub num: ConfigRange<u8>,

    /// The musical phrase configuration for this track
    pub phrase: ConfigPhrase,

    /// The synthesizer configuration for this track
    pub synth: ConfigSynth,
}