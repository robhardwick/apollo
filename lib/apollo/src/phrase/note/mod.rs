mod adsr;

use rand::{
    RngCore,
    SeedableRng
};
use rand::rngs::SmallRng;

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

    pub fn from_config(config: ConfigNote, seed: u64, sample_rate: f32) -> Self {
        let mut rng = SmallRng::seed_from_u64(seed);

        let length = config.length.random(&mut rng) * sample_rate;
        Note::new(
            length,
            config.frequency.random(&mut rng),
            ADSR::from_config(config.adsr, rng.next_u64(), length)
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