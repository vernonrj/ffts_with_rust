mod float_range;
mod generate;
mod signals;

use crate::float_range::FloatRange;

use clap::{App, Arg};
use gnuplot::{AxesCommon, Figure};
use num_complex::Complex;
use rustfft::FFTplanner;

fn main() {
    let matches = App::new("fft_pipeline")
        .arg(Arg::with_name("fft")
             .long("fft")
             .help("generate an fft"))
        .get_matches();
    let signals = signals::Sum::new()
        .add(signals::Sine { frequency: 100.0, amplitude: 2.0 })
        .add(signals::Sine { frequency: 200.0, amplitude: 1.0 });
        // .add(signals::Sawtooth { period: 0.05, amplitude: 1.0 });
    let range = FloatRange::from(0..5).with_step(0.001);
    let trace: Vec<Complex<f64>> = generate::trace(signals, range).collect();
    let mut fg = Figure::new();
    if !matches.is_present("fft") {
        fg.axes2d()
            .lines(range, trace.into_iter().map(|c| c.re), &[]);
    } else {
        let mut trace = trace;
        let mut planner = FFTplanner::new(false);
        let fft = planner.plan_fft(trace.len());
        let mut output: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0); trace.len()];
        fft.process(&mut trace, &mut output);
        {
            let rotate_len = output.len() / 2;
            // output.rotate_left(rotate_len);
            output.truncate(rotate_len);
        }
        println!("output size = {}", output.len());
        let sample_rate = range.sample_rate();
        let x_axis = FloatRange { start: 0.0, stop: sample_rate/2.0, step: 1.0 }
            .with_num_points(output.len());
        fg.axes2d()
            .set_x_label("Frequency", &[])
            .lines(x_axis, output.into_iter().map(|c| c.norm()), &[]);
    }
    fg.show();
}
