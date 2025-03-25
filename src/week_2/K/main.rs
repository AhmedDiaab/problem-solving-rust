use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Must provide values");

    let input: Vec<u64> = input
        .split_whitespace()
        .filter_map(|n| n.parse::<u64>().ok())
        .collect();

    let limit = input[0];

    let search_index = input[1];

    let odds = (limit + 1) / 2;
    let element = if search_index <= odds {
        2 * search_index - 1
    } else {
        2 * (search_index - odds)
    };

    println!("{}", element);
}
// this is a bit hard for me