use rand::Rng;
use rand::rngs::SmallRng;
use rand::distributions::uniform::SampleUniform;
use serde::Deserialize;

/// A range of values from [min](struct.ConfigRange.html#structfield.min) to
/// [max](struct.ConfigRange.html#structfield.max) that a value can be randomly generated within
/// or, alternatively, a single fixed value that will always be used
#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum ConfigRange<T> {
    Range {
        /// The minimum value that can be randomly generated
        min: T,

        /// The maximum value that can be randomly generated
        max: T,
    },
    Fixed(T),
}

impl<T> ConfigRange<T> where T: SampleUniform + Copy {
    pub fn get(&self, rng: &mut SmallRng) -> T {
        match self {
            ConfigRange::Range { min, max } => rng.gen_range(min, max),
            ConfigRange::Fixed(value) => *value,
        }
    }
}