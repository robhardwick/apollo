use failure::Error;

use apollo::{
    Apollo,
    Config,
};

pub fn run(config: Config, preset: String, sample_rate: f32) -> Result<(), Error> {
    let apollo = Apollo::new(config, preset, sample_rate)?.into_iter();

    for (clock, value) in (1..).zip(apollo) {
        println!("{} {}", clock, value);
    }

    Ok(())
}