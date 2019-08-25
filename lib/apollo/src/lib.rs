mod config;
mod phrase;
mod synth;
mod track;

use failure::Error;
use rand::SeedableRng;
use rand::rngs::SmallRng;

use phrase::Phrase;
use synth::Synth;
use track::Track;

pub use config::Config;

pub struct Apollo {
    tracks: Vec<Track>,
    size: f32,
}

impl Apollo {
    pub fn new(config: Config, preset: String, sample_rate: f32) -> Result<Self, Error> {
        let mut rng = SmallRng::seed_from_u64(config.seed);
        let preset = config.preset(preset)?;

        let tracks: Vec<Track> = preset.tracks(&mut rng).into_iter().map(|(seed, track)| {
            Track::new(
                Phrase::new(track.phrase, seed, sample_rate),
                Synth::new(track.synth, sample_rate)
            )
        }).collect();
        let size = tracks.len() as f32;

        Ok(Apollo {
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