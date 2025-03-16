use std::io;

fn main() {
    let mut input_line = String::new();

    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read numbers");

    let input_line: Vec<i32> = input_line
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    if input_line.iter().any(|&x| !(1..=1000).contains(&x)) {
        println!("Some values are out of range!");
        return;
    }

    let max_bricks_load_count: i32 = (input_line[0] - (input_line[0] % input_line[1])) / input_line[1];

    println!("{}", max_bricks_load_count);
}