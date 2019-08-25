pub mod note;

use rand::{
    RngCore,
    SeedableRng
};
use rand::rngs::SmallRng;

use crate::config::phrase::ConfigPhrase;
use note::{
    Note,
    NoteSample,
};

pub struct Phrase {
    notes: Vec<Note>,
    length: usize,
    clock: f32,
    position: usize,
}

impl Phrase {
    pub fn new(config: ConfigPhrase, seed: u64, sample_rate: f32) -> Self {
        let mut rng = SmallRng::seed_from_u64(seed);

        let notes: Vec<Note> = (0..config.length.random(&mut rng)).map(|_| {
            Note::from_config(config.note.clone(), rng.next_u64(), sample_rate).into_iter()
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