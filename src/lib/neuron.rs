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
    #[allow(clippy::new_ret_no_self)]
    pub fn new(previous: Vec<Rc<RefCell<Neuron>>>, next: Vec<Rc<RefCell<Neuron>>>,
               weights: Vec<f64>, bias: f64, activation: Activation, ) -> Neuron {

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
    
    /// # Panics
    pub fn feedforward(&mut self) {
        let mut sum = 0.0;
        for (input, weight) in self.inputs.iter().zip(self.weights.iter()) {
            sum += input * weight;
        }
        sum += self.bias;

        let activated_value = self.activation.apply_scalar(sum).unwrap();

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
                Neuron::Hidden(hidden) => hidden.inputs.clone_from(&self.inputs.clone()),
                Neuron::Input(input) => input.inputs.clone_from(&self.inputs.clone()),
                Neuron::Output(output) => output.inputs.clone_from(&self.inputs.clone()),
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
    Sigmoid,
    Tanh,
    ReLU,
    Identity,
    Softmax,
}

impl Activation {
    pub fn apply_scalar(&self, x: f64) -> Result<f64, String> {
        match self {
            Activation::Sigmoid => Ok(sigmoid(x)),
            Activation::Tanh => Ok(tanh(x)),
            Activation::ReLU => Ok(relu(x)),
            Activation::Identity => Ok(identity(x)),
            Activation::Softmax => Err("This Activation function does not operate on scalars".to_string())
        }
    }

    pub fn apply_vector(&self, x: Vec<f64>) -> Result<Vec<f64>, String> {
        match self {
            Activation::Softmax => Ok(softmax(x)),
            _ => Err("This Activation function does not operate on vectors".to_string())
        }
    }
}

pub enum Neuron {
    Hidden(HiddenNeuron),
    Input(InputNeuron),
    Output(OutputNeuron),
}


