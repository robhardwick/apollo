use std::f32::consts::PI;
use std::fmt;

use crate::config::synth::ConfigSynthSignal;

const TWO_PI: f32 = 2. * PI;

#[derive(Debug)]
pub enum Signal {
    Sine {
        offset: f32,
    },
    Saw {
        offset: f32,
    },
    Square {
        offset: f32,
    },
}

impl Signal {
    pub fn new(config: &ConfigSynthSignal, offset: f32) -> Self {
        match config {
            ConfigSynthSignal::Sine => Signal::Sine { offset },
            ConfigSynthSignal::Saw => Signal::Saw { offset },
            ConfigSynthSignal::Square => Signal::Square { offset },
        }
    }

    pub fn sample(&self, phase: f32) -> f32 {
        match self {
            Signal::Sine { offset } => ((phase + offset) * TWO_PI).sin(),
            Signal::Saw { offset } => (phase + offset) - (phase + offset).floor(),
            Signal::Square { offset } => if ((phase + offset) * TWO_PI).sin() < 0.5 { 1.0 } else { -1.0 },
        }
    }
}

impl fmt::Display for Signal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Signal::Sine { offset } => write!(f, "sine({})", offset),
            Signal::Saw { offset } => write!(f, "saw({})", offset),
            Signal::Square { offset } => write!(f, "square({})", offset),
        }
    }
}