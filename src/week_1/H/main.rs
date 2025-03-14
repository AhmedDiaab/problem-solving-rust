use std::io::stdin;

fn main() {
    let mut inputs: String = String::new();

    stdin().read_line(&mut inputs).expect("Must provide input");

    let inputs: Vec<i32> = inputs
        .split_whitespace()
        .filter_map(|v| v.parse::<i32>().ok())
        .collect();

    if inputs.iter().any(|&f| f < 0 && f > 1000) {
        println!("Must provide values between 1 and 1000");
    }

    let factor: f32 = inputs[1] as f32 / 100 as f32;

    let kcal: f32 = inputs[0] as f32 * factor as f32;

    println!("{}", kcal);
}
