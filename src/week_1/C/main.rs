use std::io::stdin;

fn main() {
    let mut line = String::new();

    stdin().read_line(&mut line).expect("Line not provided");

    let mut numbers: Vec<i32> = line
        .split_whitespace()
        .filter_map(|n| n.parse::<i32>().ok())
        .collect();

    let total = numbers.remove(0);

    if !(100..=200).contains(&total) {
        println!("out of bounds total");
        return;
    }

    if numbers.iter().any(|&n| !(1..=100).contains(&n)) {
        println!("out of bounds number");
        return;
    }

    let total = total - numbers[0] + numbers[1];

    println!("{}", total);
}
