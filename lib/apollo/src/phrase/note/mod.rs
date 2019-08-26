mod adsr;

use crate::config::phrase::note::ConfigNote;
use adsr::ADSR;

#[derive(Debug)]
pub struct Note {
    pub length: f32,
    frequency: f32,
    adsr: ADSR,
}

#[derive(Debug)]
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