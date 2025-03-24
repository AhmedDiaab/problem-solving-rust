use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Must provide values");

    let string_number: Vec<String> = input.split_whitespace().filter_map(|f| f.parse::<String>().ok()).collect();

    let string_number = string_number[0].to_string();
    let string_number_reversed: String = string_number.chars().rev().collect();

    if string_number_reversed == string_number {
        println!("Yes");
        return;
    }

    println!("No");
}