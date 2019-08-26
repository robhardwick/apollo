use rand::Rng;
use rand::rngs::SmallRng;
use rand::distributions::uniform::SampleUniform;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Range<T> {
    min: T,
    max: T,
}

impl<T> Range<T> where T: SampleUniform + Copy {
    pub fn random(&self, rng: &mut SmallRng) -> T {
        rng.gen_range(self.min, self.max)
    }
}