mod error;

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
    notes: Vec<f32>,
    rng: SmallRng,
    distribution: WeightedIndex<u8>,
}

impl Scale {
    pub fn new(config: &ConfigScale, seed: u64) -> Result<Self, ScaleError> {
        let rng = SmallRng::seed_from_u64(seed);
        let notes = config.notes.clone();

        let weights = notes.iter().enumerate().map(|(idx, _)| if idx == 0 { 2 } else { 1 });
        let distribution = WeightedIndex::new(weights).map_err(|_| ScaleError::DistributionCreate)?;

        Ok(Scale {
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