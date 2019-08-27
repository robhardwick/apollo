//! # Apollo
//!
//! A library for generative musical composition and synthesis. Each generated song is an instance
//! of the [Apollo](struct.Apollo.html) struct which is initialised with the following data:
//! 
//! - A [Config](struct.Config.html)
//! - A [ConfigPreset](struct.ConfigPreset.html)
//! - A seed value for the random number generator
//! - A sample rate
//! 
//! An example invocation that outputs 2 seconds of samples to the console:
//! 
//! ```
//! use serde_json;
//! use apollo::{
//!     Apollo,
//!     Config,
//! };
//! # use failure::Error;
//! # fn main() -> Result<(), Error> {
//! 
//! // Create basic configuration from JSON
//! let config: Config = serde_json::from_str(r#"{
//!     "seed": 1,
//!     "presets": [{
//!         "id": "default",
//!         "rhythm": {
//!             "bpm": {"min": 60, "max": 120},
//!             "beat": {"min": 3, "max": 5},
//!             "unit": [4, 8],
//!             "weight": "shorter"
//!         },
//!         "scale": ["major"],
//!         "tracks": [{
//!             "num": {"min": 1, "max": 3},
//!             "phrase": {
//!                 "length": {"min": 1, "max": 4},
//!                 "note": {
//!                     "frequency": {"min": 200.0, "max": 800.0},
//!                     "adsr": {
//!                         "amplitude": {"min": 0.5, "max": 1.0},
//!                         "attack": {"min": 0.05, "max": 0.4},
//!                         "release": {"min": 0.1, "max": 0.2}
//!                     }
//!                 }
//!             },
//!             "synth": {
//!                 "num": {"min": 1, "max": 3},
//!                 "offset": {"min": 0.0, "max": 100.0},
//!                 "signal": ["sine", "saw", "square"]
//!             }
//!         }]
//!     }],
//!     "scales": [{
//!         "name": "C Major",
//!         "tags": ["major"],
//!         "notes": [261.63, 293.66, 329.63, 349.23, 392.00, 440.00, 493.88]
//!     }]
//! }"#)?;
//! let preset = String::from("default");
//! let sample_rate = 44100;
//! let length = 2 * sample_rate; // 2 seconds
//! let seed = 1;
//! 
//! // Create sample iterator using "default" configuration preset
//! let apollo = Apollo::new(config, preset, seed, sample_rate as f32)?.into_iter();
//! 
//! // Output 2 seconds worth of clock and sample values to console
//! for (clock, value) in (0..length).zip(apollo) {
//!     println!("{} {}", clock, value);
//! }
//! # Ok(())
//! # }
//! ```

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

pub use config::choice::ConfigChoice;
pub use config::error::ConfigError;
pub use config::preset::ConfigPreset;
pub use config::phrase::ConfigPhrase;
pub use config::phrase::note::ConfigNote;
pub use config::phrase::note::adsr::ConfigADSR;
pub use config::range::ConfigRange;
pub use config::rhythm::ConfigRhythm;
pub use config::rhythm::ConfigRhythmWeight;
pub use config::scale::ConfigScale;
pub use config::synth::ConfigSynth;
pub use config::synth::ConfigSynthSignal;
pub use config::track::ConfigTrack;

/// An iterator that produces audio samples
#[derive(Debug)]
pub struct Apollo {
    seed: u64,
    rhythm: Rhythm,
    scale: Scale,
    tracks: Vec<Track>,
    size: f32,
}

impl Apollo {
    /// Creates a new sample generator based on the specified [Config](struct.Config.html),
    /// preset identifier, random number generator seed value and sample rate.
    pub fn new(config: Config, preset: String, seed: u64, sample_rate: f32) -> Result<Self, Error> {
        let mut rng = SmallRng::seed_from_u64(seed);

        let preset = config.preset(preset)?;
        let rhythm = Rhythm::new(&preset.rhythm, rng.next_u64(), sample_rate)?;
        let scale_config = config.scale(&preset, &mut rng)?;
        let scale = Scale::new(&scale_config, rng.next_u64())?;

        let tracks = preset.tracks.iter()
            .flat_map(|track| (0..track.num.random(&mut rng)).map(move |_| track))
            .collect::<Vec<_>>()
            .iter()
            .map(|track| {
                Ok(Track::new(
                    Phrase::new(&track.phrase, rng.next_u64(), &rhythm, &scale),
                    Synth::new(&track.synth, rng.next_u64(), sample_rate)?
                ))
            })
            .collect::<Result<Vec<_>, Error>>()?;
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