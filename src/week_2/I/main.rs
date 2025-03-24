use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Must pass value");

    let nums_string = [
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "Greater than 9",
    ];

    let input: Vec<i64> = input
        .split_whitespace()
        .filter_map(|n| n.parse::<i64>().ok())
        .collect();

    let input = input[0];

    let idx: usize = if input >= 0 && input <= 9 {
        input as usize
    } else {
        10
    };

    println!("{}", nums_string[idx]);
}
