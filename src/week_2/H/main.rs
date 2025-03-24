use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Must provide input");

    let input: Vec<u64> = input.split_whitespace().filter_map(|n| n.parse::<u64>().ok()).collect();

    let k = input[0];
    let x = input[1];

    let total = k * 500;

    if total >= x {
        println!("Yes");
        return;
    }

    println!("No");
}