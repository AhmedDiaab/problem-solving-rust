use std::io::stdin;

fn main() {
    let mut input_line = String::new();

    stdin()
        .read_line(&mut input_line)
        .expect("Numbers not provided");

    let input_line: Vec<i32> = input_line
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    if input_line.iter().any(|&x| !(1..=100).contains(&x)) {
        println!("Some values are out of bound!");
        return;
    }

    let product = input_line[0] * input_line[1];

    println!("{}" , product);
}
