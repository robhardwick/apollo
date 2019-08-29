pub mod note;

use std::fmt;

use rand::{
    RngCore,
    SeedableRng
};
use rand::rngs::SmallRng;

use crate::config::phrase::ConfigPhrase;
use crate::rhythm::Rhythm;
use crate::scale::Scale;
use note::{
    Note,
    NoteSample,
};

#[derive(Debug)]
pub struct Phrase {
    notes: Vec<Note>,
    length: usize,
    clock: f32,
    position: usize,
}

impl Phrase {
    pub fn new(config: &ConfigPhrase, seed: u64, rhythm: &Rhythm, scale: &Scale) -> Self {
        let mut rng = SmallRng::seed_from_u64(seed);

        let notes: Vec<Note> = (0..config.length.get(&mut rng)).flat_map(|_| {
            let mut rng = SmallRng::seed_from_u64(rng.next_u64());
            rhythm.bar(&mut rng)
                .into_iter()
                .zip(scale.clone().into_iter())
                .map(move |(length, frequency)| {
                    Note::from_config(&config.note, rng.next_u64(), length, frequency).into_iter()
                })
        }).collect();
        let length = notes.len();

        Phrase {
            notes,
            length,
            clock: 0.,
            position: 0,
        }
    }
}

impl Iterator for Phrase {
    type Item = NoteSample;

    fn next(&mut self) -> Option<NoteSample> {
        self.clock += 1.;
        if self.clock > self.notes[self.position].length {
            self.clock = 0.;

            self.position += 1;
            if self.position == self.length {
                self.position = 0;
            }
        }

        self.notes[self.position].next()
    }
}

impl fmt::Display for Phrase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} notes", self.length)
    }
}