mod float_range;
mod generate;
mod signals;

use crate::float_range::FloatRange;
use gnuplot::Figure;
use num_complex::Complex;

fn main() {
    let signals = signals::Sum::new()
        .add(signals::Sine { frequency: 0.5, amplitude: 1.0 })
        .add(signals::Sine { frequency: 1.0, amplitude: 1.0 });
        // .add(signals::Sawtooth { period: 1.0, amplitude: 1.0 });
    let range = FloatRange::from(0..4).with_step(0.001);
    let trace: Vec<Complex<f64>> = generate::trace(signals, range).collect();
    let mut fg = Figure::new();
    fg.axes2d()
        .lines(range, trace.into_iter().map(|c| c.re), &[]);
    fg.show();
}
