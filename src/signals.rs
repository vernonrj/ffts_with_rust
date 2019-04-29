use std::f64::consts::PI;
use std::ops::{Bound, RangeBounds};

use crate::generate::Generator;
use num_complex::Complex;


/**
 * Add multiple signals together
 */
#[derive(Default)]
pub struct Sum {
    signals: Vec<Box<Generator>>,
}

impl Sum {
    pub fn new() -> Self { Sum { signals: Vec::new() } }
    pub fn add<G: Generator + 'static>(mut self, gen: G) -> Self {
        self.signals.push(Box::new(gen));
        self
    }
}

impl Generator for Sum {
    fn output(&self, time: f64) -> Complex<f64> {
        self.signals.iter().map(|s| s.output(time.clone())).sum::<Complex<f64>>()
    }
}

/**
 * Clip in a range
 */
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Clip<G: Generator, R: RangeBounds<f64>> {
    pub range: R,
    pub gen: G,
}

impl<G, R> Clip<G, R>
    where G: Generator,
          R: RangeBounds<f64>,
{
    pub fn new(gen: G, range: R) -> Self {
        Clip { range, gen, }
    }
}

impl<G, R> Generator for Clip<G, R> 
    where G: Generator,
          R: RangeBounds<f64>,
{
    fn output(&self, time: f64) -> Complex<f64> {
        let unclipped: Complex<f64> = self.gen.output(time);
        let (mag, phase) = unclipped.to_polar();
        let mag = match self.range.start_bound() {
            Bound::Unbounded => mag,
            Bound::Excluded(&bnd) if mag <= bnd => bnd,
            Bound::Included(&bnd) if mag < bnd => bnd,
            _ => mag,
        };
        let mag = match self.range.end_bound() {
            Bound::Unbounded => mag,
            Bound::Excluded(&bnd) if mag >= bnd => bnd,
            Bound::Included(&bnd) if mag > bnd => bnd,
            _ => mag,
        };
        Complex::from_polar(&mag, &phase)
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
    fn output(&self, time: f64) -> Complex<f64> {
        ((time * self.frequency * 2.0 * PI).sin() * self.amplitude).into()
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
    fn output(&self, time: f64) -> Complex<f64> {
        let time = time % self.period;
        let rising = |time| time * self.amplitude / self.period * 4.0;
        let falling = |time| -1.0 * rising(time);
        if time <= self.period / 4.0 {
            // rising from 0..amplitude
            rising(time).into()
        } else if time <= 3.0 * self.period / 4.0 {
            // dropping from amplitude..-amplitude
            (falling(time - self.period / 4.0) + self.amplitude).into()
        } else {
            // rising from -amplitude..0
            (rising(time - 3.0 * self.period / 4.0) - self.amplitude).into()
        }
    }
}

#[test]
fn test_sin_default() {
    let s = Sine { frequency: 1.0, amplitude: 1.0 };
    let epsilon = 0.0001;
    assert!(s.output(0.0).re - 0.0 < epsilon);
    assert!(s.output(0.25).re - 1.0 < epsilon);
    assert!(s.output(0.5).re - 0.0 < epsilon);
    assert!(s.output(0.75).re - -1.0 < epsilon);
    assert!(s.output(1.0).re - 0.0 < epsilon);
}

#[test]
fn test_sin_frequency() {
    let s = Sine { frequency: 2.0, amplitude: 1.0 };
    let epsilon = 0.0001;
    assert!(s.output(0.0).re - 0.0 < epsilon);
    assert!(s.output(0.125).re - 1.0 < epsilon);
    assert!(s.output(0.25).re - 0.0 < epsilon);
    assert!(s.output(0.375).re - -1.0 < epsilon);
    assert!(s.output(0.5).re - 0.0 < epsilon);
}

#[test]
fn test_sin_amplitude() {
    let s = Sine { frequency: 1.0, amplitude: 10.0 };
    let epsilon = 0.0001;
    assert!(s.output(0.0).re - 0.0 < epsilon);
    assert!(s.output(0.25).re - 10.0 < epsilon);
    assert!(s.output(0.5).re - 0.0 < epsilon);
    assert!(s.output(0.75).re - -10.0 < epsilon);
    assert!(s.output(1.0).re - 0.0 < epsilon);
}


#[test]
fn test_sawtooth_default() {
    let s = Sawtooth { period: 1.0, amplitude: 1.0 };
    assert_eq!(s.output(0.0).re, 0.0);
    assert_eq!(s.output(0.25).re, 1.0);
    assert_eq!(s.output(0.5).re, 0.0);
    assert_eq!(s.output(0.75).re, -1.0);
    assert_eq!(s.output(1.0).re, 0.0);
}

#[test]
fn test_sawtooth_period() {
    let s = Sawtooth { period: 2.0, amplitude: 1.0 };
    assert_eq!(s.output(0.0).re, 0.0);

    assert_eq!(s.output(0.25).re, 0.5);
    assert_eq!(s.output(0.5).re, 1.0);
    assert_eq!(s.output(0.75).re, 0.5);

    assert_eq!(s.output(1.0).re, 0.0);

    assert_eq!(s.output(1.25).re, -0.5);
    assert_eq!(s.output(1.5).re, -1.0);
    assert_eq!(s.output(1.75).re, -0.5);

    assert_eq!(s.output(2.0).re, 0.0);
}

#[test]
fn test_sawtooth_amplitude() {
    let s = Sawtooth { period: 1.0, amplitude: 10.0 };
    assert_eq!(s.output(0.0).re, 0.0);
    assert_eq!(s.output(0.25).re, 10.0);
    assert_eq!(s.output(0.5).re, 0.0);
    assert_eq!(s.output(0.75).re, -10.0);
    assert_eq!(s.output(1.0).re, 0.0);
}
