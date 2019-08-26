mod config;
mod phrase;
mod scale;
mod synth;
mod rhythm;
mod track;

use std::fmt;

use failure::Error;
use rand::{
    RngCore,
    SeedableRng,
};
use rand::rngs::SmallRng;

use phrase::Phrase;
use rhythm::Rhythm;
use scale::Scale;
use synth::Synth;
use track::Track;

pub use config::Config;

#[derive(Debug)]
pub struct Apollo {
    seed: u64,
    rhythm: Rhythm,
    scale: Scale,
    tracks: Vec<Track>,
    size: f32,
}

impl Apollo {
    pub fn new(config: Config, preset: String, seed: u64, sample_rate: f32) -> Result<Self, Error> {
        let mut rng = SmallRng::seed_from_u64(seed);

        let preset = config.preset(preset)?;
        let rhythm = Rhythm::new(&preset.rhythm, rng.next_u64(), sample_rate)?;
        let scale_config = config.scale(&preset, &mut rng)?;
        let scale = Scale::new(&scale_config, rng.next_u64())?;

        let tracks: Vec<Track> = preset.tracks.iter().flat_map(|track| {
            let mut rng = SmallRng::seed_from_u64(rng.next_u64());
            (0..track.num.random(&mut rng)).map(|_| {
                Track::new(
                    Phrase::new(&track.phrase, rng.next_u64(), &rhythm, &scale),
                    Synth::new(&track.synth, sample_rate)
                )
            }).collect::<Vec<_>>()
        }).collect();
        let size = tracks.len() as f32;

        Ok(Apollo {
            seed,
            rhythm,
            scale,
            tracks,
            size,
        })
    }
}

impl Iterator for Apollo {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let sample: f32 = self.tracks.iter_mut()
            .filter_map(|track| track.next())
            .sum();
        Some(sample / self.size)
    }
}

impl fmt::Display for Apollo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Seed: {}\nRhythm: {}\nScale: {}\nTracks: {}", self.seed, self.rhythm, self.scale, self.size)
    }
}