mod error;
mod filter;
mod signal;

use std::fmt;

use rand::SeedableRng;
use rand::rngs::SmallRng;

use crate::config::synth::ConfigSynth;
use crate::phrase::note::NoteSample;
use error::SynthError;
use filter::Filter;
use signal::Signal;

#[derive(Debug)]
pub struct Synth {
    sample_rate: f32,
    signals: Vec<Signal>,
    filter: Filter,
    size: f32,
    clock: f32,
}

impl Synth {
    pub fn new(config: &ConfigSynth, seed: u64, sample_rate: f32) -> Result<Self, SynthError> {
        let mut rng = SmallRng::seed_from_u64(seed);
        let mut offset = 0f32;
        let signals: Vec<Signal> = (0..config.num.get(&mut rng)).map(|_| {
            let signal = config.signal.get(&mut rng).map_or_else(
                || Err(SynthError::SignalType),
                |signal| Ok(Signal::new(signal, offset))
            );
            offset += config.offset.get(&mut rng);
            signal
        }).collect::<Result<Vec<_>, _>>()?;
        let filter = Filter::new(
            config.filter.get(&mut rng).ok_or(SynthError::FilterType)?,
            &mut rng,
            sample_rate
        );
        let size = signals.len() as f32;

        Ok(Synth {
            sample_rate,
            signals,
            filter,
            size,
            clock: 0.,
        })
    }

    pub fn sample(&mut self, note: NoteSample) -> f32 {
        self.clock += 1.;
        let phase = self.clock * note.frequency / self.sample_rate;
        let samples: f32 = self.signals.iter()
            .map(|signal| signal.sample(phase))
            .sum();
        self.filter.sample(samples / self.size) * note.amplitude
    }
}

impl fmt::Display for Synth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for signal in self.signals.iter() {
            write!(f, "{} ", signal)?;
        }
        write!(f, "{}", self.filter)
    }
}