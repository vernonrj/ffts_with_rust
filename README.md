# Playing with rust

This repo plays around with FFTs and plotting.

## Running

Clone this repo and `cd` into it. Run with `cargo run`. Probably requires some sort of plotting library to be installed.

There are a few signals to play with (the signals are hard-coded): Two sine waves, a clipped sine wave, and a sawtooth. Pass either `sine`, `clip`, or `sawtooth` to plot one of the three.

You can also adjust the number of seconds of data generated (with `--time`) or the sample rate (with `--sample_rate`)
