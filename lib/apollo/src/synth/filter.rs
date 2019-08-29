use std::fmt;
use std::f32::consts::E;

use rand::rngs::SmallRng;

use crate::config::synth::ConfigSynthFilter;

#[derive(Debug)]
pub enum Filter {
    FourPole {
        frequency: f32,
        resonance: f32,
        params: (f32, f32, f32),
        state: [f32; 8],
    },
}

impl Filter {
    pub fn new(config: &ConfigSynthFilter, rng: &mut SmallRng, sample_rate: f32) -> Self {
        match config {
            ConfigSynthFilter::FourPole { frequency, resonance } => {
                let frequency = frequency.get(rng);
                let resonance = resonance.get(rng);

                let f = 2. * frequency / sample_rate;
                let k = 3.6 * f - 1.6 * f * f - 1.;
                let p = (k + 1.) * 0.5;
                let scale = E.powf((1. - p) * 1.386249);
                let r = resonance * scale;

                Filter::FourPole {
                    frequency,
                    resonance,
                    params: (r, p, k),
                    state: [0., 0., 0., 0., 0., 0., 0., 0.],
                }
            },
        }
    }

    pub fn sample(&mut self, sample: f32) -> f32 {
        match self {
            Filter::FourPole { frequency: _, resonance: _, params, state } => {
                let (r, p, k) = params;

                let x = sample - *r * state[3];
                state[0] = x * *p + state[4] * *p - *k * state[0];
                state[1] = state[0] * *p + state[5] * *p - *k * state[1];
                state[2] = state[1] * *p + state[6] * *p - *k * state[2];
                state[3] = state[2] * *p + state[7] * *p - *k * state[3];
                state[3] = state[3] - state[3].powf(3.) / 6.;

                state[4] = x;
                state[5] = state[0];
                state[6] = state[1];
                state[7] = state[2];

                state[3]
            },
        }
    }
}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Filter::FourPole { frequency, resonance, params: _, state: _ } => {
                write!(f, "filter({:.2}, {:.2})", frequency, resonance)
            },
        }
    }
}