use std::ops::Sub;

use nalgebra::*;

const E: f64 = 2.71828;
fn sigmoid(neuron: f64) -> f64 { 
    1.0/(1.0 + E.powf(-neuron))
}

fn sigmoid_prime(neuron: f64) -> f64 {
    let sig: f64 = sigmoid(neuron);
    sig * (1.0 - sig)
}

const LEARNING_RATE: f64 = 0.025;

const INPUT_NODES: usize = 784;
const HIDDEN_NODES: usize = 20;
const OUTPUT_NODES: usize = 10;

pub struct NeuralNetwork {
    pub input_layer: SMatrix<f64, 1, INPUT_NODES>,
    
    pub h_weights1: SMatrix<f64, INPUT_NODES, HIDDEN_NODES>,
    pub h_layer: SMatrix<f64, 1, HIDDEN_NODES>,
    pub h_weights2: SMatrix<f64, HIDDEN_NODES, OUTPUT_NODES>,

    pub output_layer: SMatrix<f64, 1, OUTPUT_NODES>
}

pub struct MNIST {
    pub input:  SMatrix<f64, 1, INPUT_NODES>,
    pub expected: SMatrix<f64, 1, OUTPUT_NODES>
}

impl NeuralNetwork {
    fn forward_propagation(&mut self, input: SMatrix<f64, 1, INPUT_NODES>) -> SMatrix<f64, 1, OUTPUT_NODES> {
        self.input_layer = input;
        self.h_layer = (input*self.h_weights1).map(|n: f64| sigmoid(n));
        self.output_layer = (self.h_layer*self.h_weights2).map(|n: f64| sigmoid(n));
 
        return self.output_layer;
    }

    fn backward_propagation(&mut self, expected: SMatrix<f64, 1, OUTPUT_NODES>) {
        let output_layer_delta: SMatrix<f64, 1, OUTPUT_NODES> = expected - self.output_layer;

        let output_layer_error: SMatrix<f64, 1, OUTPUT_NODES> = output_layer_delta.component_mul(&self.output_layer.map(|n: f64| sigmoid_prime(n)));
        let h_layer_error: SMatrix<f64, 1, HIDDEN_NODES> = (output_layer_delta * self.h_weights2.transpose()).component_mul(&self.h_layer.map(|n| sigmoid_prime(n)));
        //let h_layer_error: SMatrix<f64, 1, HIDDEN_NODES> = (output_layer_delta * self.h_weights2.transpose()).component_mul(&self.h_layer);

        let delta_weights2: SMatrix<f64, HIDDEN_NODES, OUTPUT_NODES> = LEARNING_RATE * (self.h_layer.transpose() * output_layer_error);
        let delta_weights1: SMatrix<f64, INPUT_NODES, HIDDEN_NODES> = LEARNING_RATE * (self.input_layer.transpose() * h_layer_error);

        self.h_weights2 += delta_weights2;
        self.h_weights1 += delta_weights1;
    }

    pub fn fit(&mut self, data_set: &[MNIST], epoch: i16, epsilon: f64) {
        'epoch_loop:for _ in 0..=epoch {
            let mut MSE: f64 = 0.0;

            for data in data_set {
                let predict: SMatrix<f64, 1, OUTPUT_NODES> = self.predict(data.input);
                self.backward_propagation(data.expected);

                MSE += ((data.expected.sub(predict)).map(|n: f64| (2.0).powf(n))).sum();
            }
            MSE = MSE / ((data_set.len() * OUTPUT_NODES) as f64);
            print!("{}\n", MSE);
            if MSE < epsilon { break 'epoch_loop }
        }
    }

    pub fn predict(&mut self, input: SMatrix<f64, 1, INPUT_NODES>) -> SMatrix<f64, 1, OUTPUT_NODES> {
        self.forward_propagation(input)
    }
}