use std::io::{stdin};

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Must provide a value");

    let input: Vec<u32> = input
        .split_whitespace()
        .filter_map(|v| v.parse::<u32>().ok())
        .collect();

    let input: u32 = input[0];

    if input < 1 || input > 100 {
        println!("Must be between 1 and 100 inclusive");
        return;
    }

    let is_even = if input % 2 == 0 && input > 2 {
        String::from("YES")
    } else {
        String::from("NO")
    };

    println!("{}", is_even);
}
