use crate::numbrs;
use core::str;
use numbrs::randfloat;
use std::fs::File;
use std::io::{self, Read, Write};

#[allow(dead_code)]
pub struct Perceptron {
    weights: Vec<f64>,
    bias: f64,
    learning_rate: f64,
    epochs: i32,
    threshold: f64,
}

#[allow(dead_code)]
impl Perceptron {
    pub fn new(n: i32, seed: u64) -> Perceptron {
        let weights = randfloat(-1.0, 1.0, n, seed);
        let bias = 1.0;
        let learning_rate = 0.01;
        let epochs = 0;
        let threshold = 0.5;
        Perceptron {
            weights,
            bias,
            learning_rate,
            epochs,
            threshold,
        }
    }

    pub fn import(name: &str) -> Perceptron {
        let mut file = File::open(name).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let weights: Vec<f64> = serde_json::from_str(&contents).unwrap();
        let bias = 1.0;
        let learning_rate = 0.01;
        let epochs = 0;
        let threshold = 0.5;
        Perceptron {
            weights,
            bias,
            learning_rate,
            epochs,
            threshold,
        }
    }

    pub fn show_weights(&self) {
        println!("Weights: {:?}", self.weights);
    }

    fn heaviside_step_function(&self, x: f64) -> i32 {
        if x >= self.threshold {
            return 1;
        }
        return 0;
    }

    pub fn fit(&mut self, x: Vec<Vec<f64>>, y: Vec<i32>, epochs: i32) {
        for _ in 0..epochs {
            for i in 0..x.len() {
                let y_hat = self.predict(x[i].clone());
                for j in 0..self.weights.len() {
                    self.weights[j] = self.adjust_weights(self.weights[j], y[i], y_hat, x[i][j]);
                }
            }
        }
    }

    pub fn predict(&self, x: Vec<f64>) -> i32 {
        let mut sum = 0.0;
        for i in 0..self.weights.len() {
            sum += self.weights[i] * x[i];
        }
        sum += self.bias;
        return self.heaviside_step_function(sum);
    }

    pub fn adjust_weights(&self, w: f64, y: i32, y_hat: i32, x: f64) -> f64 {
        return w + self.learning_rate * (y - y_hat) as f64 * x;
    }

    pub fn export(&self, name: &str) -> io::Result<()> {
        let weights = serde_json::to_string(&self.weights).unwrap();
        let mut file = File::create(name)?;
        file.write_all(weights.as_bytes())?;
        Ok(())
    }
}
//
