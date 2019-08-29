mod adsr;

use crate::config::phrase::note::ConfigNote;
use adsr::ADSR;

pub const EMPTY_NOTE: NoteSample = NoteSample {
    frequency: 0.,
    amplitude: 0.,
};

#[derive(Debug)]
pub struct Note {
    pub length: f32,
    frequency: f32,
    adsr: ADSR,
}

#[derive(Debug, PartialEq)]
pub struct NoteSample {
    pub frequency: f32,
    pub amplitude: f32,
}

impl Note {
    pub fn new(length: f32, frequency: f32, adsr: ADSR) -> Self {
        Note {
            length,
            frequency,
            adsr: adsr.into_iter(),
        }
    }

    pub fn from_config(config: &ConfigNote, seed: u64, length: f32, frequency: f32) -> Self {
        Note::new(
            length,
            frequency,
            ADSR::from_config(&config.adsr, seed, length)
        )
    }
}

impl Iterator for Note {
    type Item = NoteSample;

    fn next(&mut self) -> Option<NoteSample> {
        self.adsr.next().and_then(|amplitude| Some(NoteSample {
            frequency: self.frequency,
            amplitude,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::adsr::ADSR;
    use super::*;

    #[test]
    fn test_playing() {
        let mut note = Note::new(5., 220., ADSR::new(1., 2., 0., 1., 0.));
        for _ in 0..10 {
            assert_eq!(note.next(), Some(NoteSample { frequency: 220., amplitude: 1. }));
        }
    }

    #[test]
    fn test_not_playing() {
        let mut note = Note::new(5., 220., ADSR::new(1., 2., 0., 0., 0.));
        for _ in 0..10 {
            assert_eq!(note.next(), None);
        }
    }
}