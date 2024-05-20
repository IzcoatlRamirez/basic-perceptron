mod dataframe;
mod numbrs;
mod perceptron;
use numbrs::metrics::accuracy_score;
use numbrs::{randfloat, randint};
use perceptron::Perceptron;

fn main() {
    let n_examples = 100;
    let seed = 80;
    //let n_features = 4;

    let ages = randfloat(20.0, 70.0, n_examples, seed);
    let cholesterol_levels = randfloat(150.0, 300.0, n_examples, seed);
    let systolic_bp = randfloat(110.0, 180.0, n_examples, seed);
    let bmi = randfloat(18.0, 35.0, n_examples, seed);
    let target = randint(0, 1, n_examples, seed);

    /*la aleatoriedad no permite encontrar un hiperplano que separe linealmente los datos*/

    let x = vec![ages, cholesterol_levels, systolic_bp, bmi];
    let y = target.clone();

    let (x_train, x_test, y_train, _y_test) = dataframe::simple_split(x, y, 0.7);

    // let mut perceptron = Perceptron::new(n_features, seed);

    // perceptron.fit(x_train, y_train, 50);

    let perceptron = Perceptron::import("model.json");

    let mut test = Vec::new();
    for i in 0..x_test.len() {
        test.push(perceptron.predict(x_test[i].clone()));
    }

    println!("Accuracy: {}", accuracy_score(_y_test, test));
    //let _ = perceptron.export("model.json");
}
