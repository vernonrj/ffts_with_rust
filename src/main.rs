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
        .arg(Arg::with_name("signal")
             .takes_value(true)
             .required(true)
             .possible_value("sine")
             .possible_value("sawtooth"))
        .get_matches();
    let signals = match matches.value_of("signal") {
        Some("sine") => signals::Sum::new()
            .add(signals::Sine { frequency: 100.0, amplitude: 2.0 })
            .add(signals::Sine { frequency: 200.0, amplitude: 1.0 }),
        Some("sawtooth") => signals::Sum::new()
            .add(signals::Sawtooth { period: 0.5, amplitude: 1.0 }),
        Some(e) => panic!("unknown option: {}", e),
        None => panic!("no signal given"),
    };
    let range = FloatRange::from(0..1).with_step(1e-3);
    let trace: Vec<Complex<f64>> = generate::trace(signals, range).collect();
    let mut fg = Figure::new();
    fg.axes2d()
        .set_pos_grid(1, 2, 0)
        .lines(range, trace.clone().into_iter().map(|c| c.re), &[]);
    let mut planner = FFTplanner::new(false);
    let fft = planner.plan_fft(trace.len());
    let mut output: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0); trace.len()];
    fft.process(&mut trace.clone(), &mut output);
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
        .set_pos_grid(1, 2, 1)
        .lines(x_axis, output.into_iter().map(|c| c.norm()), &[]);
    fg.show();
}
