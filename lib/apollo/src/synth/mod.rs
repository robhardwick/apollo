mod error;
mod signal;

use rand::SeedableRng;
use rand::rngs::SmallRng;

use crate::config::synth::ConfigSynth;
use crate::phrase::note::NoteSample;
use error::SynthError;
use signal::Signal;

#[derive(Debug)]
pub struct Synth {
    sample_rate: f32,
    signals: Vec<Signal>,
    size: f32,
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
        let size = signals.len() as f32;

        Ok(Synth {
            sample_rate,
            signals,
            size,
        })
    }

    pub fn sample(&self, clock: f32, note: NoteSample) -> f32 {
        let phase = clock * note.frequency / self.sample_rate;
        let samples: f32 = self.signals.iter()
            .map(|signal| signal.sample(phase))
            .sum();
        (samples / self.size) * note.amplitude
    }
}