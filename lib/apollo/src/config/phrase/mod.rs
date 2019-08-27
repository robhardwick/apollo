pub mod note;

use serde::Deserialize;

use crate::config::range::ConfigRange;
use note::ConfigNote;

/// A musical phrase configuration consisting of a series of notes in one or more bars
#[derive(Clone, Debug, Deserialize)]
pub struct ConfigPhrase {
    /// A range of values for the number of bars in the phrase
    pub length: ConfigRange<u8>,

    /// The configuration for notes in this phrase
    pub note: ConfigNote,
}