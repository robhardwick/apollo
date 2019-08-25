pub mod note;

use serde::Deserialize;

use crate::config::range::Range;
use note::ConfigNote;

#[derive(Clone, Debug, Deserialize)]
pub struct ConfigPhrase {
    pub length: Range<u8>,
    pub note: ConfigNote,
}