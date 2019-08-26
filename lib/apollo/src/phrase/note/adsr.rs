use rand::SeedableRng;
use rand::rngs::SmallRng;

use crate::config::phrase::note::adsr::ConfigADSR;

#[derive(Debug)]
pub struct ADSR {
    amplitude: f32,
    length: f32,
    attack: f32,
    // TODO: Implement decay
    //decay: f32,
    sustain: f32,
    release: f32,
    clock: f32,
}

impl ADSR {
    pub fn new(amplitude: f32, length: f32, attack: f32, sustain: f32, release: f32) -> Self {
        ADSR {
            amplitude,
            length,
            attack: attack * length,
            sustain: sustain * length,
            release: release * length,
            clock: 0.,
        }
    }

    pub fn from_config(config: &ConfigADSR, seed: u64, length: f32) -> Self {
        let mut rng = SmallRng::seed_from_u64(seed);

        let amplitude = config.amplitude.random(&mut rng);
        let attack = config.attack.random(&mut rng);
        let release = config.release.random(&mut rng);
        let sustain = 1. - attack - release;
        Self::new(
            amplitude,
            length,
            attack,
            attack + sustain,
            attack + sustain + release
        )
    }
}

impl Iterator for ADSR {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        self.clock += 1.;
        if self.clock > self.length {
            self.clock = 0.;
        }

        // Attack
        if self.clock <= self.attack {
            Some(self.amplitude * (self.clock / self.attack))

        // Sustain
        } else if self.clock <= self.sustain {
            Some(self.amplitude)

        // Release
        } else if self.clock <= self.release {
            Some(self.amplitude * (1. - ((self.clock - self.sustain) / (self.release - self.sustain))))

        // Finished
        } else {
            None
        }
    }
}