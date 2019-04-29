mod float_range;
mod generate;
mod signals;

use crate::float_range::FloatRange;
use gnuplot::Figure;

fn main() {
    let signals = signals::Sum::new()
        .add(signals::Sine { frequency: 0.5, amplitude: 1.0 })
        .add(signals::Sine { frequency: 1.0, amplitude: 1.0 });
        // .add(signals::Sawtooth { period: 1.0, amplitude: 1.0 });
    let range = FloatRange::from(0..4).with_step(0.001);
    let trace = generate::trace(signals, range);
    let mut fg = Figure::new();
    fg.axes2d()
        .lines(range, trace, &[]);
    fg.show();
}
