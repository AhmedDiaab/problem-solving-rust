use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Must provide input");

    let problems_count: u16 = input.trim().parse().unwrap();

    let mut sum: u16 = 0;
    
    if problems_count > 1000 {
        println!("Count between 1 and 1000");
        return;
    }

    for _ in 1..=problems_count {
        input.clear();
        stdin().read_line(&mut input).expect("Must provide input");

        let total: u8 = input
            .trim()
            .split_whitespace()
            .filter_map(|p| p.parse::<u8>().ok())
            .into_iter().sum();

        if total >= 2 {
            sum += 1;
        }
    }

    println!("{}", sum);
}
