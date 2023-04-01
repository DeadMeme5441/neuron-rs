use plotly::layout::{GridPattern, LayoutGrid, RowOrder};
use plotly::{Layout, Plot, Scatter};
use rand::prelude::*;
use std::borrow;
use std::cell::RefCell;
use std::rc::Rc;
use std::thread::sleep;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Dunit {
    pub inputs: Vec<Dendrite>,
    pub potential: Vec<f64>,
    pub threshold: f64,
    pub refract: bool,
    pub activated: bool,
    pub refraction_time: usize,
    pub activation_time: usize,
}

#[derive(Debug)]
pub struct Sunit {
    pub input: Dendrite,
    pub output: Dendrite,
}

#[derive(Debug)]
pub struct Nunit {
    pub inputs: Vec<Dendrite>,
    pub potential: Vec<f64>,
    pub threshold: f64,
    pub refract: bool,
    pub activated: bool,
    pub refraction_time: usize,
    pub activation_time: usize,
}

#[derive(Debug)]
pub struct Dendrite {
    pub unit: Rc<RefCell<Dunit>>,
}

#[derive(Debug)]
pub struct Synapse {
    pub unit: Rc<RefCell<Sunit>>,
}

#[derive(Debug)]
pub struct Neuron {
    pub unit: Rc<RefCell<Nunit>>,
}

impl Dendrite {
    pub fn new() -> Self {
        Self {
            unit: Rc::new(RefCell::new(Dunit {
                inputs: Vec::new(),
                potential: [-70.0].to_vec(),
                threshold: -55.0,
                refract: false,
                activated: false,
                refraction_time: 0,
                activation_time: 0,
            })),
        }
    }
}

impl Neuron {
    pub fn new() -> Self {
        Self {
            unit: Rc::new(RefCell::new(Nunit {
                inputs: Vec::new(),
                potential: [-70.0].to_vec(),
                threshold: -55.0,
                refract: false,
                activated: false,
                refraction_time: 0,
                activation_time: 0,
            })),
        }
    }

    pub fn compute(&mut self, time: usize) {
        let mut neuron = self.unit.as_ref().borrow_mut();
        let mut potential: f64 = neuron.potential[time];

        let input: f64 = neuron
            .inputs
            .iter()
            .map(|signal| signal.unit.as_ref().borrow_mut().potential[time])
            .sum();

        if neuron.activated == true {
            let new_activation = time;
            if new_activation - neuron.activation_time < 3 {
                potential += 47.5;
            }
            if new_activation - neuron.activation_time >= 3
                && new_activation - neuron.activation_time < 6
            {
                potential -= 40.0;
            } else if new_activation - neuron.activation_time >= 6 {
                neuron.refract = true;
                neuron.activated = false;
                neuron.refraction_time = time;
            }
        } else {
            if neuron.refract == false {
                if input > neuron.threshold {
                    neuron.activated = true;
                    neuron.activation_time = time;
                    potential = -55.0;
                } else {
                }
            } else if neuron.refract == true {
                let new_refraction = time;
                potential += 15.5;
                if new_refraction - neuron.refraction_time < 3 {
                    neuron.refract = false;
                    potential = -70.0;
                }
            }
        }
        neuron.potential.push(potential);
    }

    pub fn plot(&mut self) {
        let neuron = self.unit.as_ref().borrow();

        let mut plots = Vec::new();
        let inputs: Vec<Vec<f64>> = neuron
            .inputs
            .iter()
            .map(|x| x.unit.as_ref().borrow().potential.clone())
            .collect();

        for input in inputs {
            plots.push(input);
        }

        plots.push(neuron.potential.clone());

        let mut plot = Plot::new();

        for elem in &plots {
            let trace = Scatter::new((0..elem.len()).collect::<Vec<usize>>(), elem.to_vec());
            plot.add_trace(trace);
        }

        let layout = Layout::new().grid(
            LayoutGrid::new()
                .rows(plots.len())
                .columns(1)
                .pattern(GridPattern::Independent)
                .row_order(RowOrder::TopToBottom),
        );
        plot.set_layout(layout);

        plot.write_html("out.html");
    }
}
