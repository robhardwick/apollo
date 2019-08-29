use std::fmt;

use crate::phrase::Phrase;
use crate::phrase::note::EMPTY_NOTE;
use crate::synth::Synth;

#[derive(Debug)]
pub struct Track {
    phrase: Phrase,
    synth: Synth,
}

impl Track {
    pub fn new(phrase: Phrase, synth: Synth) -> Self {
        Track {
            phrase: phrase.into_iter(),
            synth,
        }
    }
}

impl Iterator for Track {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let note = self.phrase.next().unwrap_or(EMPTY_NOTE);
        Some(self.synth.sample(note))
    }
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.synth, self.phrase)
    }
}