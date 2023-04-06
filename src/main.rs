mod neuron;
use neuron::{Dendrite, Neuron, Synapse};

use plotly::layout::{GridPattern, LayoutGrid};
use plotly::{Layout, Plot, Scatter};
use rand::prelude::*;
use std::rc::Rc;
use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    let input_one: Vec<f64> = generate_logic_wave(10, 100);
    let action_potential_one: Vec<f64> = action_potential_input_encoder([input_one].to_vec());

    plot_graph([action_potential_one.clone()].to_vec());

    let dendrite_one: Dendrite = Dendrite::new();
    dendrite_one.unit.as_ref().borrow_mut().potential = action_potential_one;

    let input_two: Vec<f64> = generate_logic_wave(20, 100);
    let action_potential_two: Vec<f64> = action_potential_input_encoder([input_two].to_vec());

    let dendrite_two: Dendrite = Dendrite::new();
    dendrite_two.unit.as_ref().borrow_mut().potential = action_potential_two;

    let synapse_one: Synapse = Synapse::new();

    synapse_one
        .unit
        .as_ref()
        .borrow_mut()
        .inputs
        .push(Rc::clone(&dendrite_one.unit));

    let synapse_two: Synapse = Synapse::new();
    synapse_two
        .unit
        .as_ref()
        .borrow_mut()
        .inputs
        .push(Rc::clone(&dendrite_two.unit));

    let mut dendrite_three: Dendrite = Dendrite::new();

    dendrite_three
        .unit
        .as_ref()
        .borrow_mut()
        .inputs
        .push(Rc::clone(&synapse_one.unit));

    let mut dendrite_four: Dendrite = Dendrite::new();

    dendrite_four
        .unit
        .as_ref()
        .borrow_mut()
        .inputs
        .push(Rc::clone(&synapse_two.unit));

    let mut neuron_one: Neuron = Neuron::new();

    neuron_one
        .unit
        .as_ref()
        .borrow_mut()
        .inputs
        .push(Rc::clone(&dendrite_three.unit));

    neuron_one
        .unit
        .as_ref()
        .borrow_mut()
        .inputs
        .push(Rc::clone(&dendrite_four.unit));

    for time in 0..100 {
        synapse_one.compute(time);
        synapse_two.compute(time);
        dendrite_three.compute(time);
        dendrite_four.compute(time);
        neuron_one.compute(time);
    }

    // let mut plot = Plot::new();
    // let trace = Scatter::new((0..out.len()).collect::<Vec<usize>>(), out);
    // plot.add_trace(trace);
    // plot.write_html("out.html");

    neuron_one.plot();
}

fn action_potential_input_encoder(input_signals: Vec<Vec<f64>>) -> Vec<f64> {
    let mut potential: f64 = -70.0;
    let threshold: f64 = 0.5;
    let mut refract: bool = false;
    let mut activated: bool = false;
    let mut output_signal: Vec<f64> = Vec::new();
    let mut reft = Instant::now();
    let mut activation = Instant::now();

    for time in 0..input_signals[0].len() {
        let input: f64 = input_signals.iter().map(|signal| signal[time]).sum();

        if activated == true {
            let new_activation = Instant::now();
            if new_activation.duration_since(activation).as_secs_f64() < 0.02 {
                potential = 40.0;
            }
            if new_activation.duration_since(activation).as_secs_f64() >= 0.02
                && new_activation.duration_since(activation).as_secs_f64() < 0.04
            {
                potential -= 60.0;
            } else if new_activation.duration_since(activation).as_secs_f64() >= 0.04 {
                refract = true;
                activated = false;
                reft = Instant::now();
            }
        } else {
            if refract == false {
                potential += input;
                if input >= threshold {
                    activation = Instant::now();
                    activated = true;
                    potential = -55.0;
                } else {
                    potential -= input;
                }
            } else if refract == true {
                let new_reft = Instant::now();
                potential += 15.5;
                if new_reft.duration_since(reft).as_secs_f64() < 0.03 {
                    refract = false;
                    potential = -70.0;
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

    for elem in &plots {
        let trace = Scatter::new((0..elem.len()).collect::<Vec<usize>>(), elem.to_vec());
        plot.add_trace(trace);
    }

    let layout = Layout::new().grid(
        LayoutGrid::new()
            .rows(plots.len())
            .columns(1)
            .pattern(GridPattern::Independent),
    );
    plot.set_layout(layout);

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
            inps.push(0.6);
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
