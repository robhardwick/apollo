use crate::phrase::Phrase;
use crate::synth::Synth;

#[derive(Debug)]
pub struct Track {
    clock: f32,
    phrase: Phrase,
    synth: Synth,
}

impl Track {
    pub fn new(phrase: Phrase, synth: Synth) -> Self {
        Track {
            clock: 0.,
            phrase: phrase.into_iter(),
            synth,
        }
    }
}

impl Iterator for Track {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        self.clock += 1.;
        self.phrase.next().and_then(|note| Some(self.synth.sample(self.clock, note)))
    }
}