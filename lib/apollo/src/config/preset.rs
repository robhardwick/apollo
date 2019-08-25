use rand::{
    Rng,
    RngCore,
    SeedableRng
};
use rand::rngs::SmallRng;
use rand::distributions::Standard;
use serde::Deserialize;

use crate::config::track::ConfigTrack;

#[derive(Clone, Debug, Deserialize)]
pub struct ConfigPreset {
    pub id: String,
    pub tracks: Vec<ConfigTrack>,
}

impl ConfigPreset {
    pub fn tracks(&self, rng: &mut SmallRng) -> Vec<(u64, ConfigTrack)> {
        self.tracks.iter().flat_map(|track| {
            let mut rng = SmallRng::seed_from_u64(rng.next_u64());
            let num = track.num.random(&mut rng) as usize;
            rng.sample_iter(Standard).take(num).map(move |seed: u64| (seed, track.clone()))
        }).collect()
    }
}