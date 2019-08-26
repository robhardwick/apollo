mod error;

use rand::SeedableRng;
use rand::rngs::SmallRng;
use rand::distributions::{
    Distribution,
    WeightedIndex
};

use crate::config::rhythm::{
    ConfigRhythm,
    ConfigRhythmWeight,
};
use error::RhythmError;

const BASE_BEAT_UNIT: f32 = 4.;
const SECONDS_PER_MINUTE: f32 = 60.;

#[derive(Clone, Debug)]
pub struct Rhythm {
    sample_rate: f32,
    bpm: f32,
    beat: usize,
    beat_length: usize,
    phrase_length: usize,
    lengths: Vec<usize>,
    distribution: WeightedIndex<usize>,
}

impl Rhythm {
    pub fn new(config: &ConfigRhythm, seed: u64, sample_rate: f32) -> Result<Self, RhythmError> {
        let mut rng = SmallRng::seed_from_u64(seed);
        let bpm = config.bpm.random(&mut rng) as f32;
        let beat = config.beat.random(&mut rng) as usize;
        let unit = config.unit.random(&mut rng).ok_or(RhythmError::UnitChoose)? as f32;

        let beat_length = ((SECONDS_PER_MINUTE / bpm) * (BASE_BEAT_UNIT / unit) * sample_rate) as usize;
        let phrase_length = beat_length * beat;

        let lengths = 1..beat;
        let weights: Vec<usize> = match config.weight {
            ConfigRhythmWeight::Longer => lengths.clone().collect(),
            ConfigRhythmWeight::Shorter => lengths.clone().rev().collect(),
        };

        let distribution = WeightedIndex::new(weights).map_err(|_| RhythmError::DistributionCreate)?;
        let lengths: Vec<usize> = lengths.collect();

        Ok(Rhythm {
            sample_rate,
            bpm,
            beat,
            beat_length,
            phrase_length,
            lengths,
            distribution,
        })
    }

    pub fn phrase(&self, rng: &mut SmallRng) -> Vec<f32> {
        let mut current_length = 0usize;
        let mut phrase: Vec<f32> = Vec::with_capacity(self.beat);

        while current_length < self.phrase_length  {
            let beat = self.lengths[self.distribution.sample(rng)] * self.beat_length;
            if self.phrase_length >= current_length + beat {
                phrase.push(beat as f32);
                current_length += beat;
            }
        }

        phrase
    }
}