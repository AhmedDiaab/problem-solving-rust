use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Must provide values");

    let input = input.split_whitespace().next().unwrap_or("");

    let mut counter = 0;
    input
        .split("S")
        .filter(|pred| !pred.is_empty())
        .for_each(|str| counter = str.len());

    println!("{}", counter);
}
