use rand::Rng;
use rand::rngs::SmallRng;
use rand::distributions::uniform::SampleUniform;
use serde::Deserialize;

/// A range of values from [min](struct.ConfigRange.html#structfield.min) to
/// [max](struct.ConfigRange.html#structfield.max) that a value can be randomly generated within
#[derive(Clone, Debug, Deserialize)]
pub struct ConfigRange<T> {
    /// The minimum value that can be randomly generated
    pub min: T,

    /// The maximum value that can be randomly generated
    pub max: T,
}

impl<T> ConfigRange<T> where T: SampleUniform + Copy {
    pub fn random(&self, rng: &mut SmallRng) -> T {
        rng.gen_range(self.min, self.max)
    }
}