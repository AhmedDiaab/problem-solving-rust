use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Must pass values");

    let input: Vec<u8> = input
        .split_whitespace()
        .filter_map(|n| n.parse::<u8>().ok())
        .collect();

    let input = input[0];

    if input > 100 {
        return;
    }

    let target_score = if input >= 0 && input < 40 {
        40
    } else if input >= 40 && input < 70 {
        70
    } else if input >= 70 && input < 90 {
        90
    } else {
        100
    };

    

    let score_needed = target_score - input;

    if target_score == 100 &&  score_needed <= 10 {
        println!("expert");
        return;
    }

    println!("{}", score_needed);
}
