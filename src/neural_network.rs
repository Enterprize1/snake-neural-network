use ndarray::{Array2, Axis};
use ndarray_rand::RandomExt;
use rand::distributions::Uniform;

pub struct NeuralNetwork {
    weights: Vec<Array2<f64>>,
    biases: Vec<Array2<f64>>,
}

impl NeuralNetwork {
    pub fn new(input_size: usize, output_size: usize) -> Self {
        NeuralNetwork {
            weights: vec![Array2::random((input_size, output_size), Uniform::new(-1.0, 1.0))],
            biases: vec![Array2::zeros((1, output_size))],
        }
    }

    pub fn add_hidden_layer(&mut self, neurons: usize, after: usize) {
        assert!(after < self.weights.len());

        let input_size = self.weights[after].shape()[0];
        let output_size = self.weights[after].shape()[1];

        self.weights[after] = Array2::random((input_size, neurons), Uniform::new(-1.0, 1.0));
        self.weights.insert(after + 1, Array2::random((neurons, output_size), Uniform::new(-1.0, 1.0)));
        self.biases.insert(after, Array2::zeros((1, neurons)));
    }

    fn relu(&self, x: Array2<f64>) -> Array2<f64> {
        x.mapv(|a| if a > 0.0 { a } else { 0.01 * a })
    }

    fn relu_derivative(&self, x: &Array2<f64>) -> Array2<f64> {
        x.mapv(|a| if a > 0.0 { 1.0 } else { 0.01 })
    }

    pub fn forward(&self, x: &Array2<f64>) -> Vec<Array2<f64>> {
        let mut activations = Vec::new();
        let mut current_x = x.clone();

        for i in 0..self.weights.len() {
            current_x = self.relu(current_x.dot(&self.weights[i]) + &self.biases[i]);
            activations.push(current_x.clone());
        }

        activations
    }

    pub fn backward(&mut self, x: Array2<f64>, y: Array2<f64>, activations: Vec<Array2<f64>>, learning_rate: f64) {
        let mut error = activations.last().unwrap() - &y;

        for i in (0..self.weights.len()).rev() {
            let input = if i == 0 { x.view() } else { activations[i-1].view() };

            // Calculate gradients
            let d_weights = input.t().dot(&error);
            let d_biases = error.sum_axis(Axis(0));

            // Update weights and biases
            self.weights[i] -= &(d_weights * learning_rate);
            self.biases[i] -= &(d_biases * learning_rate);

            // Propagate the error to the previous layer, unless it's the first layer
            if i > 0 {
                error = error.dot(&self.weights[i].t()) * self.relu_derivative(&activations[i-1]);
            }
        }
    }
}
