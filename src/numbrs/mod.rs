use rand::Rng;

pub fn randfloat(low: f64, high: f64, n: i32) -> Vec<f64> {
    let mut numbers = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        let random_number = rng.gen_range(low..high);
        numbers.push(random_number);
    }
    return numbers;
}

pub fn randint(low: i32, high: i32, n: i32) -> Vec<i32> {
    let mut numbers = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        let random_number = rng.gen_range(low..high+1);
        numbers.push(random_number);
    }
    return numbers;
}


/*struct metrics */