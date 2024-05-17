pub fn simple_split(
    x: Vec<Vec<f64>>,
    y: Vec<i32>,
    train_ratio: f64,
) -> (Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<i32>, Vec<i32>) {
    let mut x_train: Vec<Vec<f64>> = Vec::new();
    let mut x_test: Vec<Vec<f64>> = Vec::new();

    let n = x[0].len() as f64 * train_ratio;
    for i in 0..x.len() {
        x_train.push(x[i][0..n as usize].to_vec());
        x_test.push(x[i][(n as usize)..].to_vec());
    }

    let y_train = y[0..n as usize].to_vec();
    let y_test = y[(n as usize)..].to_vec();

    (x_train, x_test, y_train, y_test)
}
