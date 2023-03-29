use plotly::{Plot, Scatter};
use rand::prelude::*;
use std::thread::sleep;
use std::time::{Duration, Instant};

struct Dendrite {
    uid: f64,
    potential: f64,
    threshold: f64,
    refract: bool,
    activated: bool,
    refraction_time: f64,
    activation_time: f64,
}

fn action_potential_generation(input_signals: Vec<Vec<f64>>) -> Vec<f64> {
    let mut potential: f64 = 0.0;
    let threshold: f64 = 0.6;
    let mut refract: bool = false;
    let mut activated: bool = false;
    let mut output_signal: Vec<f64> = Vec::new();
    let mut reft = Instant::now();
    let mut activation = Instant::now();

    for time in 0..input_signals[0].len() {
        let input: f64 = input_signals.iter().map(|signal| signal[time]).sum();

        if activated == true {
            let new_activation = Instant::now();
            if new_activation.duration_since(activation).as_secs_f64() < 0.03 {
                potential += 0.4;
            }
            if new_activation.duration_since(activation).as_secs_f64() > 0.03
                && new_activation.duration_since(activation).as_secs_f64() < 0.06
            {
                potential -= 0.4;
            } else if new_activation.duration_since(activation).as_secs_f64() > 0.07 {
                refract = true;
                activated = false;
                reft = Instant::now();
            }
        } else {
            if refract == false {
                potential += input;
                if potential > threshold {
                    activation = Instant::now();
                    activated = true;
                    potential = 0.2;
                } else {
                    potential -= input;
                }
            } else if refract == true {
                let new_reft = Instant::now();
                potential += 0.05;
                if new_reft.duration_since(reft).as_secs_f64() < 0.03 {
                    refract = false;
                    potential = 0.0;
                }
            }
        }
        output_signal.push(potential);
        sleep(Duration::new(0, 10000000));
    }
    output_signal
}

fn plot_graph(plots: Vec<Vec<f64>>) {
    let mut plot = Plot::new();

    for elem in plots {
        let trace = Scatter::new((0..elem.len()).collect::<Vec<usize>>(), elem);
        plot.add_trace(trace);
    }

    plot.write_html("out.html");
}

fn generate_random_signal(time: usize) -> Vec<f64> {
    let mut inps: Vec<f64> = Vec::new();
    let mut rng = rand::thread_rng();
    for t in 0..time {
        inps.push(0.5 * rng.gen::<f64>());
    }
    inps
}

fn generate_XOR_output(input_signals: Vec<Vec<f64>>) -> Vec<f64> {
    let mut out_wave: Vec<f64> = Vec::new();
    for time in 0..input_signals[0].len() {
        if input_signals[0][time] == 0.32 && input_signals[1][time] == 0.32 {
            out_wave.push(0.0);
        } else if input_signals[0][time] == 0.32 {
            out_wave.push(1.0);
        } else if input_signals[1][time] == 0.32 {
            out_wave.push(1.0);
        } else {
            out_wave.push(0.0);
        }
    }

    out_wave
}

fn generate_logic_wave(interval: usize, time: usize) -> Vec<f64> {
    let mut inps: Vec<f64> = Vec::new();
    for t in 0..time {
        if t.rem_euclid(interval) == 0 && t != 0 {
            inps.push(0.32);
        } else {
            inps.push(0.0);
        }
    }
    inps
}

fn generate_cos_wave_based_on_time(time: usize) -> Vec<f64> {
    let mut out_wave: Vec<f64> = Vec::new();

    let e = std::f64::consts::E;
    let now = Instant::now();
    for t in 0..time {
        // println!("{:?}", theta_wave(t as f64 / 2.0));
        let new_now = Instant::now();
        out_wave.push(theta_wave(new_now.duration_since(now).as_secs_f64()));
        sleep(Duration::new(0, 10000000));
    }

    out_wave
}

fn theta_wave(time: f64) -> f64 {
    let constant: f64 = (2.0 * std::f64::consts::PI) / 1.0;
    let output: f64 = 1.0 * (constant * time).cos();

    output
}
