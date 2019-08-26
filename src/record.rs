use failure::Error;
use hound::{
    WavSpec,
    WavWriter,
    SampleFormat,
};

use apollo::{
    Apollo,
    Config,
};

const DEFAULT_BITS_PER_SAMPLE: u16 = 16;

pub fn run(config: Config, preset: String, seed: u64, sample_rate: f32, length: usize, filename: &String) -> Result<(), Error> {
    let apollo = Apollo::new(config, preset, seed, sample_rate)?.into_iter();
    println!("{}", apollo);

    let spec = WavSpec {
        channels: 1,
        sample_rate: sample_rate as u32,
        bits_per_sample: DEFAULT_BITS_PER_SAMPLE,
        sample_format: SampleFormat::Int,
    };

    let mut writer = WavWriter::create(filename, spec)?;
    for (_, value) in (0..length).zip(apollo) {
        let value = (value * std::i16::MAX as f32) as i16;
        writer.write_sample(value)?;
    }

    println!("\nFinished writing to '{}'", filename);
    Ok(())
}