use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

pub fn randfloat(low: f64, high: f64, n: i32, seed: u64) -> Vec<f64> {
    let mut numbers = Vec::new();
    let mut rng = StdRng::seed_from_u64(seed);
    for _ in 0..n {
        let random_number = rng.gen_range(low..high);
        numbers.push(random_number);
    }
    return numbers;
}

pub fn randint(low: i32, high: i32, n: i32, seed: u64) -> Vec<i32> {
    let mut numbers = Vec::new();
    let mut rng = StdRng::seed_from_u64(seed);
    for _ in 0..n {
        let random_number = rng.gen_range(low..high + 1);
        numbers.push(random_number);
    }
    return numbers;
}

pub mod metrics {
    pub fn accuracy_score(y_true: Vec<i32>, y_pred: Vec<i32>) -> f64 {
        let mut correct = 0;
        for i in 0..y_true.len() {
            if y_true[i] == y_pred[i] {
                correct += 1;
            }
        }
        return correct as f64 / y_true.len() as f64;
    }
}
