use std::f64::consts::PI;

use crate::generate::Generator;


/**
 * Add multiple signals together
 */
#[derive(Default)]
pub struct Sum {
    signals: Vec<Box<Generator>>,
}

impl Sum {
    pub fn new() -> Self { Self::default() }
    pub fn add<G: Generator + 'static>(mut self, gen: G) -> Self {
        self.signals.push(Box::new(gen));
        self
    }
}

impl Generator for Sum {
    fn output(&self, time: f64) -> f64 {
        self.signals.iter().map(|s| s.output(time)).sum::<f64>()
    }
}


/**
 * Sine wave generator
 */
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Sine {
    pub frequency: f64,
    pub amplitude: f64,
}

impl Generator for Sine {
    fn output(&self, time: f64) -> f64 {
        (time * self.frequency * 2.0 * PI).sin() * self.amplitude
    }
}


/**
 * Sawtooth generator
 */
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Sawtooth {
    pub period: f64,
    pub amplitude: f64,
}

impl Generator for Sawtooth {
    fn output(&self, time: f64) -> f64 {
        let time = time % self.period;
        let rising = |time| time * self.amplitude / self.period * 4.0;
        let falling = |time| -1.0 * rising(time);
        if time <= self.period / 4.0 {
            // rising from 0..amplitude
            rising(time)
        } else if time <= 3.0 * self.period / 4.0 {
            // dropping from amplitude..-amplitude
            falling(time - self.period / 4.0) + self.amplitude
        } else {
            // rising from -amplitude..0
            rising(time - 3.0 * self.period / 4.0) - self.amplitude
        }
    }
}

#[test]
fn test_sin_default() {
    let s = Sine { frequency: 1.0, amplitude: 1.0 };
    let epsilon = 0.0001;
    assert!(s.output(0.0) - 0.0 < epsilon);
    assert!(s.output(0.25) - 1.0 < epsilon);
    assert!(s.output(0.5) - 0.0 < epsilon);
    assert!(s.output(0.75) - -1.0 < epsilon);
    assert!(s.output(1.0) - 0.0 < epsilon);
}

#[test]
fn test_sin_frequency() {
    let s = Sine { frequency: 2.0, amplitude: 1.0 };
    let epsilon = 0.0001;
    assert!(s.output(0.0) - 0.0 < epsilon);
    assert!(s.output(0.125) - 1.0 < epsilon);
    assert!(s.output(0.25) - 0.0 < epsilon);
    assert!(s.output(0.375) - -1.0 < epsilon);
    assert!(s.output(0.5) - 0.0 < epsilon);
}

#[test]
fn test_sin_amplitude() {
    let s = Sine { frequency: 1.0, amplitude: 10.0 };
    let epsilon = 0.0001;
    assert!(s.output(0.0) - 0.0 < epsilon);
    assert!(s.output(0.25) - 10.0 < epsilon);
    assert!(s.output(0.5) - 0.0 < epsilon);
    assert!(s.output(0.75) - -10.0 < epsilon);
    assert!(s.output(1.0) - 0.0 < epsilon);
}


#[test]
fn test_sawtooth_default() {
    let s = Sawtooth { period: 1.0, amplitude: 1.0 };
    assert_eq!(s.output(0.0), 0.0);
    assert_eq!(s.output(0.25), 1.0);
    assert_eq!(s.output(0.5), 0.0);
    assert_eq!(s.output(0.75), -1.0);
    assert_eq!(s.output(1.0), 0.0);
}

#[test]
fn test_sawtooth_period() {
    let s = Sawtooth { period: 2.0, amplitude: 1.0 };
    assert_eq!(s.output(0.0), 0.0);

    assert_eq!(s.output(0.25), 0.5);
    assert_eq!(s.output(0.5), 1.0);
    assert_eq!(s.output(0.75), 0.5);

    assert_eq!(s.output(1.0), 0.0);

    assert_eq!(s.output(1.25), -0.5);
    assert_eq!(s.output(1.5), -1.0);
    assert_eq!(s.output(1.75), -0.5);

    assert_eq!(s.output(2.0), 0.0);
}

#[test]
fn test_sawtooth_amplitude() {
    let s = Sawtooth { period: 1.0, amplitude: 10.0 };
    assert_eq!(s.output(0.0), 0.0);
    assert_eq!(s.output(0.25), 10.0);
    assert_eq!(s.output(0.5), 0.0);
    assert_eq!(s.output(0.75), -10.0);
    assert_eq!(s.output(1.0), 0.0);
}
