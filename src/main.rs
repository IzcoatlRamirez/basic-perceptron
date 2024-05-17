mod dataframe;
mod numbrs;
mod perceptron;
//use perceptron::Perceptron;

fn main() {
    let n_features = 10;

    let ages = numbrs::randfloat(20.0, 70.0, n_features);
    let cholesterol_levels = numbrs::randfloat(150.0, 300.0, n_features);
    let systolic_bp = numbrs::randfloat(110.0, 180.0, n_features);
    let bmi = numbrs::randfloat(18.0, 35.0, n_features);
    let target = numbrs::randint(0, 1, n_features);

    let  x = vec![ages, cholesterol_levels, systolic_bp, bmi];
    let  y = target.clone();

    let (x_train, x_test, y_train, y_test) = dataframe::simple_split(x, y, 0.8);

    println!("x_train: {:?}\n\n", x_train);
    println!("x_test: {:?}\n\n", x_test);
    println!("y_train: {:?}\n\n", y_train);
    println!("y_test: {:?}\n\n", y_test);

    // let mut perceptron = Perceptron::new(n_features);
    // perceptron.show_weights();

    // perceptron.fit(x_train, y_train,50);
    // perceptron.show_weights();    

    //println!("prediction: {:?}", perceptron.predict(x_test[0].clone()));


}
