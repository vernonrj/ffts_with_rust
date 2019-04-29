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
        .arg(Arg::with_name("sample_rate")
             .long("sample_rate")
             .takes_value(true)
             .default_value("1e3")
             .validator(|value| value.parse::<f64>().map(|_| ()).map_err(|e| e.to_string()))
             .help("Sample Rate to use"))
        .arg(Arg::with_name("time")
             .long("time")
             .takes_value(true)
             .default_value("1.0")
             .validator(|value| value.parse::<f64>().map(|_| ()).map_err(|e| e.to_string()))
             .help("Amount of time to use"))
        .arg(Arg::with_name("signal")
             .takes_value(true)
             .required(true)
             .possible_value("sine")
             .possible_value("sawtooth")
             .possible_value("clip"))
        .get_matches();
    let sample_rate: f64 = matches.value_of("sample_rate").unwrap().parse().unwrap();
    let time: f64 = matches.value_of("time").unwrap().parse().unwrap();
    let range = FloatRange::from(0.0..time).with_step(1.0 / sample_rate);
    let mut fg = Figure::new();
    // choose which signal we're going to use based on user input
    let signals = match matches.value_of("signal") {
        Some("sine") => signals::Sum::new()
            .add(signals::Sine { frequency: 100.0, amplitude: 2.0 })
            .add(signals::Sine { frequency: 200.0, amplitude: 1.0 }),
        Some("clip") => signals::Sum::new()
            .add(signals::Clip::new(
                    signals::Sine {
                        frequency: 9.0, amplitude: 2.0
                    },
                    -1.0..1.0)),
        Some("sawtooth") => signals::Sum::new()
            .add(signals::Sawtooth { period: 0.5, amplitude: 1.0 }),
        Some(e) => panic!("unknown option: {}", e),
        None => panic!("no signal given"),
    };
    // Generate a trace based on the chosen signal
    let trace: Vec<Complex<f64>> = generate::trace(signals, range).collect();
    // plot the time-domain signal
    fg.axes2d()
        .set_x_label("Time (s)", &[])
        .set_pos_grid(1, 2, 0)
        .lines(range, trace.clone().into_iter().map(|c| c.re), &[]);
    // Convert trace to frequency domain
    let output = fft(trace);
    let sample_rate = range.sample_rate();
    let x_axis = FloatRange { start: 0.0, stop: sample_rate/2.0, step: 1.0 }
        .with_num_points(output.len());
    // plot the frequency-domain signal
    fg.axes2d()
        .set_x_label("Frequency (Hz)", &[])
        .set_pos_grid(1, 2, 1)
        .lines(x_axis, output.into_iter().map(|c| c.norm()), &[]);
    // finally, show the plot
    fg.show();
}


fn fft(mut trace: Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let mut planner = FFTplanner::new(false);
    let fft = planner.plan_fft(trace.len());
    let mut output: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0); trace.len()];
    fft.process(&mut trace, &mut output);
    {
        let rotate_len = output.len() / 2;
        // output.rotate_left(rotate_len);
        output.truncate(rotate_len);
    }
    output
}
