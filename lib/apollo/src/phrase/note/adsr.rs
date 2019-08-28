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

        let amplitude = config.amplitude.get(&mut rng);
        let attack = config.attack.get(&mut rng);
        let release = config.release.get(&mut rng);
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

        // Attack
        let value = if self.clock < self.attack {
            Some(self.amplitude * (self.clock / self.attack))

        // Sustain
        } else if self.clock < self.sustain {
            Some(self.amplitude)

        // Release
        } else if self.clock < self.release {
            Some(self.amplitude * (1. - ((self.clock - self.sustain) / (self.release - self.sustain))))

        // Finished
        } else {
            None
        };

        // Update clock
        self.clock += 1.;
        if self.clock >= self.length {
            self.clock = 0.;
        }

        value
    }
}

#[cfg(test)]
mod tests {
    use crate::config::range::ConfigRange;
    use super::*;

    #[test]
    fn test_attack() {
        let mut adsr = ADSR::new(1., 5., 0.8, 1., 0.);
        for _ in 0..5 {
            assert_eq!(adsr.next(), Some(0.0));
            assert_eq!(adsr.next(), Some(0.25));
            assert_eq!(adsr.next(), Some(0.5));
            assert_eq!(adsr.next(), Some(0.75));
            assert_eq!(adsr.next(), Some(1.0));
        }
    }

    #[test]
    fn test_attack_config() {
        let config = ConfigADSR {
            amplitude: ConfigRange::Fixed(1.),
            attack: ConfigRange::Fixed(0.8),
            release: ConfigRange::Fixed(0.),
        };
        let mut adsr = ADSR::from_config(&config, 1, 5.);
        for _ in 0..5 {
            assert_eq!(adsr.next(), Some(0.0));
            assert_eq!(adsr.next(), Some(0.25));
            assert_eq!(adsr.next(), Some(0.5));
            assert_eq!(adsr.next(), Some(0.75));
            assert_eq!(adsr.next(), Some(1.0));
        }
    }

    #[test]
    fn test_release() {
        let mut adsr = ADSR::new(1., 5., 0., 0.2, 1.);
        for _ in 0..5 {
            assert_eq!(adsr.next(), Some(1.));
            assert_eq!(adsr.next(), Some(1.));
            assert_eq!(adsr.next(), Some(0.75));
            assert_eq!(adsr.next(), Some(0.5));
            assert_eq!(adsr.next(), Some(0.25));
        }
    }

    #[test]
    fn test_release_config() {
        let config = ConfigADSR {
            amplitude: ConfigRange::Fixed(1.),
            attack: ConfigRange::Fixed(0.),
            release: ConfigRange::Fixed(0.8),
        };
        let mut adsr = ADSR::from_config(&config, 1, 5.);
        for _ in 0..5 {
            assert_eq!(adsr.next(), Some(1.));
            assert_eq!(adsr.next(), Some(1.));
            assert_eq!(adsr.next(), Some(0.75));
            assert_eq!(adsr.next(), Some(0.5));
            assert_eq!(adsr.next(), Some(0.25));
        }
    }

    #[test]
    fn test_no_attack_no_release() {
        let mut adsr = ADSR::new(1., 2., 0., 1., 0.);
        for _ in 0..10 {
            assert_eq!(adsr.next(), Some(1.));
        }
    }

    #[test]
    fn test_no_attack_no_release_config() {
        let config = ConfigADSR {
            amplitude: ConfigRange::Fixed(1.),
            attack: ConfigRange::Fixed(0.),
            release: ConfigRange::Fixed(0.),
        };
        let mut adsr = ADSR::from_config(&config, 1, 2.);
        for _ in 0..10 {
            assert_eq!(adsr.next(), Some(1.));
        }
    }

}