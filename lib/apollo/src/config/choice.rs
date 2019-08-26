use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Choice<T>(Vec<T>);

impl<T> Choice<T> {
    pub fn random(&self, rng: &mut SmallRng) -> Option<&T> {
        self.0.choose(rng)
    }
}