use std::{cmp::max, io::stdin};

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Input required");

    let input: Vec<i32> = input
        .split_whitespace()
        .filter_map(|d| d.parse::<i32>().ok())
        .collect();

    let a = input[0];
    let b = input[1];

    let max_num = max(a + b, max(a - b, a * b));

    println!("{}", max_num);
}
