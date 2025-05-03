Example usage of library:

```rust
/*
    Architecture:
    Input Layer: 1 neuron (Input of 1)
    Hidden Layer: 1 neuron (ReLU activation) 
    Output Layer: 1 neuron (Should output 0)
*/

let mut network = Network::from_sizes(
    vec![1, 1, 1],
    // Only two activation functions as input layer doesn't use any
    vec![
        brainstorm::lib::neuron::Activation::ReLU { f: |x| x },
        brainstorm::lib::neuron::Activation::Identity { f: |x| x },
    ],
    vec![1],
);

let input = vec![1.0];
let output = network.feedforward(input);
```