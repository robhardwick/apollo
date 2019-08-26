mod error;

use std::fmt;

use rand::SeedableRng;
use rand::rngs::SmallRng;
use rand::distributions::{
    Distribution,
    WeightedIndex
};

use crate::config::scale::ConfigScale;
use error::ScaleError;

#[derive(Clone, Debug)]
pub struct Scale {
    name: String,
    notes: Vec<f32>,
    rng: SmallRng,
    distribution: WeightedIndex<u8>,
}

impl Scale {
    pub fn new(config: &ConfigScale, seed: u64) -> Result<Self, ScaleError> {
        let rng = SmallRng::seed_from_u64(seed);
        let name = config.name.clone();
        let notes = config.notes.clone();

        let weights = notes.iter().enumerate().map(|(idx, _)| if idx == 0 { 2 } else { 1 });
        let distribution = WeightedIndex::new(weights).map_err(|_| ScaleError::DistributionCreate)?;

        Ok(Scale {
            name,
            notes,
            rng,
            distribution,
        })
    }
}

impl Iterator for Scale {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        Some(self.notes[self.distribution.sample(&mut self.rng)])
    }
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({} pitches)", self.name, self.notes.len())
    }
}