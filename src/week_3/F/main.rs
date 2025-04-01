use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Must provide input");

    let weights: Vec<u16> = input
        .trim()
        .split_whitespace()
        .filter_map(|p| p.parse::<u16>().ok())
        .collect();

    let (mut limak, mut bob) = (weights[0], weights[1]);

    let mut years_count = 0;

    while limak <= bob {
        limak *= 3;
        bob *= 2;
        years_count += 1;
    }

    println!("{}", years_count);
}
