use std::{cell::RefCell, rc::Rc};


pub struct HiddenNeuron {
    pub inputs: Vec<f64>,
    pub previous: Vec<Rc<RefCell<Neuron>>>,
    pub weights: Vec<f64>,
    pub bias: f64,
    pub activation: Activation,
    pub next: Vec<Rc<RefCell<Neuron>>>,
}

impl HiddenNeuron {
    pub fn new(previous: Vec<Rc<RefCell<Neuron>>>, 
               weights: Vec<f64>, 
               bias: f64, 
               activation: Activation, 
               next: Vec<Rc<RefCell<Neuron>>>) -> Neuron {
        Neuron::Hidden(
            HiddenNeuron {
                inputs: vec![],
                previous,
                weights,
                bias,
                activation,
                next,
            }
        )
    }

    pub fn feedforward(&mut self) {
        let mut sum = 0.0;
        for (input, weight) in self.inputs.iter().zip(self.weights.iter()) {
            sum += input * weight;
        }
        sum += self.bias;

        let activated_value = match self.activation {
            Activation::Sigmoid { f } => (f)(sum),
            Activation::Tanh { f } => (f)(sum),
            Activation::ReLU { f } => (f)(sum),
            Activation::Identity { f } => (f)(sum),
            _ => panic!("Invalid activation function for hidden layer"),        
        };

        for neuron in &self.next {
            match &mut *neuron.borrow_mut() {
                Neuron::Hidden(hidden) => hidden.inputs.push(activated_value),
                Neuron::Input(input) => input.inputs.push(activated_value),
                Neuron::Output(output) => output.inputs.push(activated_value),
            }
        }
    }
}

pub struct InputNeuron {
    pub inputs: Vec<f64>,
    pub input_dim: Vec<usize>,
    pub next: Vec<Rc<RefCell<Neuron>>>,
}

impl InputNeuron {
    pub fn new(inputs: Vec<f64>, input_dim: Vec<usize>, next: Vec<Rc<RefCell<Neuron>>>,) -> Self {
        InputNeuron { inputs, input_dim, next }
    }

    pub fn feedforward(&self){
        for neuron in &self.next {
            match &mut *neuron.borrow_mut() {
                Neuron::Hidden(hidden) => hidden.inputs = self.inputs.clone(),
                Neuron::Input(input) => input.inputs = self.inputs.clone(),
                Neuron::Output(output) => output.inputs = self.inputs.clone(),
            }
        }
    }
}

pub struct OutputNeuron {
    pub inputs: Vec<f64>,
    pub previous: Vec<Rc<RefCell<Neuron>>>,
}

impl OutputNeuron {
    pub fn new(inputs: Vec<f64>) -> Self {
        OutputNeuron { inputs, previous: vec![] }
    }

    pub fn feedforward(&self) -> f64 {
        let mut sum = 0.0;
        for input in &self.inputs {
            sum += input;
        }
        sum
    }
}


fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

fn tanh(x: f64) -> f64 {
    (x.exp() - (-x).exp()) / (x.exp() + (-x).exp())
}

fn relu(x: f64) -> f64 {
    if x < 0.0 {
        0.0
    } else {
        x
    }
}

fn identity(x: f64) -> f64 {
    x
}

fn softmax(x: Vec<f64>) -> Vec<f64> {
    let max = x.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let exps: Vec<f64> = x.iter().map(|&v| (v - max).exp()).collect();
    let sum: f64 = exps.iter().sum();
    exps.into_iter().map(|v| v / sum).collect()
}

#[derive(Clone)]
pub enum Activation {
    Sigmoid{f: fn(f64) -> f64},
    Tanh{f: fn(f64) -> f64},
    ReLU{f: fn(f64) -> f64},
    Identity{f: fn(f64) -> f64},
    Softmax{f: fn(Vec<f64>) -> Vec<f64>},
}

pub enum Neuron {
    Hidden(HiddenNeuron),
    Input(InputNeuron),
    Output(OutputNeuron),
}

#[allow(non_upper_case_globals)]
pub const Sigmoid: Activation = Activation::Sigmoid{f: sigmoid};
#[allow(non_upper_case_globals)]
pub const Tanh: Activation = Activation::Tanh{f: tanh};
#[allow(non_upper_case_globals)]
pub const ReLU: Activation = Activation::ReLU{f: relu};
#[allow(non_upper_case_globals)]
pub const Identity: Activation = Activation::Identity{f: identity};
#[allow(non_upper_case_globals)]
pub const Softmax: Activation = Activation::Softmax{f: softmax};

