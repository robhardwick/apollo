use failure::Error;

use apollo::{
    Apollo,
    Config,
};

pub fn run(config: Config, preset: String, seed: u64, sample_rate: f32, length: usize) -> Result<(), Error> {
    let apollo = Apollo::new(config, preset, seed, sample_rate)?.into_iter();

    for (clock, value) in (0..length).zip(apollo) {
        println!("{} {}", clock, value);
    }

    Ok(())
}