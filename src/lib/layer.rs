use std::{cell::RefCell, rc::Rc};

use crate::lib::neuron::{InputNeuron, Neuron, OutputNeuron, Activation};

use super::neuron::HiddenNeuron;

pub struct HiddenLayer {
    pub neurons: Vec<Rc<RefCell<Neuron>>>,
    pub size: usize,
}

pub struct InputLayer {
    pub neurons: Vec<Rc<RefCell<Neuron>>>,
    pub size: usize,
    pub inputs: Vec<f64>,
}

pub struct OutputLayer {
    pub neurons: Vec<Rc<RefCell<Neuron>>>,
    pub activation: Activation,
    pub size: usize,
}

impl HiddenLayer {
    pub fn new(size: usize, activation: Activation, next: Vec<Rc<RefCell<Neuron>>>, previous: Vec<Rc<RefCell<Neuron>>>) -> Self {

        let neurons = (0..size)
            .map(|_| {
                Rc::new(
                    RefCell::new(
                        HiddenNeuron::new(
                            previous.clone(),
                            vec![1.0],
                            0.0,
                            activation.clone(),
                            next.clone(),
                        )
                    )
                )
            })
            .collect();

        HiddenLayer { neurons, size }
    }

    pub fn feedforward(&self) {
        self.neurons.iter().for_each(|n| {
            if let Neuron::Hidden(hidden_neuron) = &mut *n.borrow_mut() {
                hidden_neuron.feedforward();
            }
        });
    }
}

impl InputLayer {
    pub fn new(size: usize, input_dim: Vec<usize>, next: Vec<Rc<RefCell<Neuron>>>) -> Self {
        let neurons = (0..size)
            .map(|_| {
                Rc::new(
                    RefCell::new(
                        Neuron::Input(
                            InputNeuron::new(
                                vec![],
                                input_dim.clone(),
                                next.clone(),
                            )
                        )
                    )
                )
            })
            .collect();

        InputLayer { neurons, size, inputs: vec![] }
    }

    pub fn feedforward(&self) {
        self.neurons.iter().for_each(|n| {
            match &mut *n.borrow_mut() {
                Neuron::Input(input_neuron) => input_neuron.feedforward(),
                _ => panic!("Unexpected neuron type in InputLayer"),
            }
        });
    }
}

impl OutputLayer {
    pub fn new(size: usize, activation: Activation) -> Self {
        let neurons = (0..size)
                    .map(|_| {
                        Rc::new(
                            RefCell::new(
                                Neuron::Output(
                                    OutputNeuron::new(
                                        vec![],
                                    )
                                )
                            )
                        )
                    })
                    .collect();
        OutputLayer { neurons, activation, size }
    }

    pub fn feedforward(&self) -> Vec<f64> {
        let output: Vec<f64> 
            = self.neurons.iter().map(|n| {
                match &mut *n.borrow_mut() {
                    Neuron::Output(output_neuron) => output_neuron.feedforward(),
                    _ => panic!("Unexpected neuron type in OutputLayer"),
                }
            }).collect();

        match self.activation {
            Activation::Sigmoid{f} 
                => output.iter().map(|&x| f(x)).collect(),
            Activation::Tanh{f} 
                => output.iter().map(|&x| f(x)).collect(),
            Activation::ReLU{f} 
                => output.iter().map(|&x| f(x)).collect(),
            Activation::Identity{f} 
                =>  output.iter().map(|&x| f(x)).collect(),
            Activation::Softmax{f} 
                => f(output),
        }
    }
}
