use crate::lib::layer::{HiddenLayer, InputLayer, OutputLayer};
use crate::lib::neuron::Neuron;
use super::neuron::Activation;


pub struct Network {
    pub input_layer: InputLayer,
    pub hidden_layers: Vec<HiddenLayer>,
    pub output_layer: OutputLayer,
}

impl Network {
    /// # Panics
    pub fn from_sizes(sizes: Vec<usize>, 
                     activations: Vec<Activation>, 
                     input_dim: Vec<usize>) -> Self{

        let mut output_layer: OutputLayer = OutputLayer::new(sizes[sizes.len() - 1], activations[activations.len() - 1].clone());
        let mut hidden_layers: Vec<HiddenLayer> = vec![];
        
        for i in (1..sizes.len() - 1).rev() {
            if i == sizes.len() - 2 {
                hidden_layers.push(
                    HiddenLayer::new(
                        sizes[i],
                        activations[i].clone(),
                        output_layer.neurons.clone(),
                        vec![]
                    )
                );
            } else {
                hidden_layers.push(
                    HiddenLayer::new(
                        sizes[i],
                        activations[i].clone(),
                        hidden_layers[i-1].neurons.clone(),
                        vec![]
                    )
                );
            }
        }

        hidden_layers.reverse();

        let input_layer = InputLayer::new(sizes[0], input_dim, hidden_layers[0].neurons.clone());
        
        for neuron in &mut output_layer.neurons {
            match &mut *neuron.borrow_mut() {
                Neuron::Output(output) => output.previous.clone_from(&hidden_layers[hidden_layers.len() - 1].neurons),
                _ => panic!("Invalid neuron type for output layer"),
            };
        }


        for i in (1..hidden_layers.len()).rev() {
            let layer = hidden_layers[i].neurons.clone();
            println!("i: {}", i);
            for neuron in layer {
                match &mut *neuron.borrow_mut() {
                    Neuron::Hidden(hidden) => hidden.previous.clone_from(&hidden_layers[i - 1].neurons),
                    _ => panic!("Invalid neuron type for hidden layer"),
                }
            }
        }

        let first_layer = hidden_layers[0].neurons.clone();
        for neuron in first_layer {
            match &mut *neuron.borrow_mut() {
                Neuron::Hidden(hidden) => hidden.previous.clone_from(&input_layer.neurons),
                _ => panic!("Invalid neuron type for hidden layer"),
            }
        }
        
        Network {
            input_layer,
            hidden_layers,
            output_layer,
        }
    }

    pub fn from_layers(
        input_layer: InputLayer,
        hidden_layers: Vec<HiddenLayer>,
        output_layer: OutputLayer,
    ) -> Self {
        Network {
            input_layer,
            hidden_layers,
            output_layer,
        }
    }
    
    /// # Panics
    pub fn feedforward(&mut self, input: Vec<f64>) -> Vec<f64> {
        assert!((input.len() == self.input_layer.size), "Input size does not match input layer size");

        self.input_layer.inputs = input;

        self.input_layer.feedforward();
        for layer in &self.hidden_layers {
            layer.feedforward();
        }
        self.output_layer.feedforward()
    }
}