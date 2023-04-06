use plotly::layout::{GridPattern, LayoutGrid, RowOrder};
use plotly::{Layout, Plot, Scatter};
use rand::prelude::*;
use std::borrow;
use std::cell::RefCell;
use std::f64::consts::E;
use std::rc::Rc;
use std::thread::sleep;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Dunit {
    pub inputs: Vec<Rc<RefCell<Sunit>>>,
    pub potential: Vec<f64>,
    pub threshold: f64,
    pub refract: bool,
    pub activated: bool,
    pub refraction_time: usize,
    pub activation_time: usize,
}

#[derive(Debug)]
pub struct Sunit {
    pub inputs: Vec<Rc<RefCell<Dunit>>>,
    pub weight: f64,
    pub value: f64,
    pub ei_type: i32,
}

#[derive(Debug)]
pub struct Nunit {
    pub inputs: Vec<Rc<RefCell<Dunit>>>,
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

    pub fn compute(&mut self, time: usize) {
        let mut dendrite = self.unit.as_ref().borrow_mut();
        let mut potential: f64 = dendrite.potential[time];

        let input: f64 = dendrite
            .inputs
            .iter()
            .map(|synapse| synapse.as_ref().borrow().value)
            .sum();

        if dendrite.activated == true {
            let new_activation = time;
            if new_activation - dendrite.activation_time < 2 {
                potential = 40.0;
            }
            if new_activation - dendrite.activation_time >= 2
                && new_activation - dendrite.activation_time < 4
            {
                potential -= 60.0;
            } else if new_activation - dendrite.activation_time >= 4 {
                dendrite.refract = true;
                dendrite.activated = false;
                dendrite.refraction_time = time;
            }
        } else {
            if dendrite.refract == false {
                if potential > dendrite.threshold {
                    dendrite.activated = true;
                    dendrite.activation_time = time;
                    potential = -55.0;
                } else {
                    if time % 3 == 0 && time != 0 {
                        potential -= 6.0;
                    } else {
                        potential += 3.0;
                    }
                    if input > 0.0 {
                        potential += 3.0;
                    }
                }
            } else if dendrite.refract == true {
                let new_refraction = time;
                potential += 15.5;
                if new_refraction - dendrite.refraction_time < 3 {
                    dendrite.refract = false;
                    potential = -70.0;
                }
            }
        }

        println!("{:?}", potential);
        dendrite.potential.push(potential);
    }
}

impl Synapse {
    pub fn new() -> Self {
        Self {
            unit: Rc::new(RefCell::new(Sunit {
                inputs: Vec::new(),
                weight: 1.0,
                value: 0.0,
                ei_type: 0,
            })),
        }
    }
    pub fn compute(&self, time: usize) {
        let mut synapse = self.unit.as_ref().borrow_mut();

        let value = synapse
            .inputs
            .iter()
            .map(|signal| signal.as_ref().borrow().potential[time] * 0.01)
            .sum::<f64>()
            * synapse.weight;

        synapse.value = -(synapse.value - value) / 2.0;

        synapse.value = (E.powf(2.0 * synapse.value) - 1.0) / (E.powf(2.0 * synapse.value) + 1.0);
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
            .map(|dendrite| dendrite.as_ref().borrow_mut().potential[time])
            .sum();

        if neuron.activated == true {
            let new_activation = time;
            if new_activation - neuron.activation_time < 2 {
                potential = 40.0;
            }
            if new_activation - neuron.activation_time >= 2
                && new_activation - neuron.activation_time < 4
            {
                potential -= 60.0;
            } else if new_activation - neuron.activation_time >= 4 {
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
                    potential = input;
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
            .map(|x| x.as_ref().borrow().potential.clone())
            .collect();

        for input in inputs {
            plots.push(input);
        }

        plots.push(neuron.potential.clone());

        let mut plot = Plot::new();

        for i in 0..plots.len() {
            let name = "trace".to_string() + &i.to_string();
            let x = "x".to_string() + &i.to_string();
            let y = "y".to_string() + &i.to_string();

            let trace = Scatter::new(
                (0..plots[i].len()).collect::<Vec<usize>>(),
                plots[i].to_vec(),
            )
            .name(name)
            .x_axis(x)
            .y_axis(y);

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
