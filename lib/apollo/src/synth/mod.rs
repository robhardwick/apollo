use std::f32::consts::PI;

use crate::config::synth::ConfigSynth;
use crate::phrase::note::NoteSample;

pub struct Synth {
    sample_rate: f32,
}

impl Synth {
    pub fn new(_config: ConfigSynth, sample_rate: f32) -> Self {
        Synth {
            sample_rate,
        }
    }

    pub fn sample(&self, clock: f32, note: NoteSample) -> f32 {
        (clock * note.frequency * 2.0 * PI / self.sample_rate).sin() * note.amplitude
    }
}