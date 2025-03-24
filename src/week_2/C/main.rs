use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Must provide input");

    let input: Vec<u16> = input
        .split_whitespace()
        .filter_map(|d| d.parse::<u16>().ok())
        .collect();

    let gold = input[0];

    let silver = input[1];

    if gold + silver == 0 {
        println!("values must be at least one of them larger than 0");
        return;
    }

    if gold > 100 || silver > 100 {
        println!("limit is 100 no more!");
        return;
    }

    let product: String = if gold == 0 {
        String::from("Silver")
    } else if silver == 0 {
        String::from("Gold")
    } else {
        String::from("Alloy")
    };

    println!("{}", product);
}
