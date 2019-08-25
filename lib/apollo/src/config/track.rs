use serde::Deserialize;

use crate::config::phrase::ConfigPhrase;
use crate::config::synth::ConfigSynth;
use crate::config::range::Range;

#[derive(Clone, Debug, Deserialize)]
pub struct ConfigTrack {
    pub num: Range<u8>,
    pub phrase: ConfigPhrase,
    pub synth: ConfigSynth,
}