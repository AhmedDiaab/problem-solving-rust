
use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Must provide input");

    let current_hour: i32 = input.trim().parse::<i32>().expect("Must provide a number");
    let hours_left = 48 - current_hour;

    println!("{hours_left}");
}