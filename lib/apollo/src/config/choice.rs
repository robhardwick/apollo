use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use serde::Deserialize;

/// A list of values from which one can be randomly selected
#[derive(Clone, Debug, Deserialize)]
pub struct ConfigChoice<T>(Vec<T>);

impl<T> ConfigChoice<T> {
    pub fn get(&self, rng: &mut SmallRng) -> Option<&T> {
        self.0.choose(rng)
    }
}