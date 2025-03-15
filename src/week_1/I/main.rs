use std::io::stdin;

fn main() {
    let mut inputs = String::new();

    stdin().read_line(&mut inputs).expect("Must provide values");

    let inputs: Vec<i32> = inputs
        .split_whitespace()
        .filter_map(|f| f.parse::<i32>().ok())
        .collect();
    
    if inputs.iter().any(|&v| v < 1 && v > 16) {
        println!("Must provide values between 1 and 16");
        return;
    }

    if inputs[0] > inputs[1] {
        println!("Must provide value 2 >= value 1");
        return;
    }

    let output = (inputs[0] * inputs[1]) / 2;

    println!("{}", output);
}
