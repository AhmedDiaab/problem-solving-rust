use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Must provide input");

    let input: Vec<u32> = input
        .split_whitespace()
        .filter_map(|i| i.parse::<u32>().ok())
        .collect();

    let mut input: u32 = input[0];

    let max_range_base: u32 = 10;
    
    let max_range: u32 = max_range_base.pow(9);

    if 1 > input || input > max_range {
        println!("Out of range");
        return;
    }

    let mut steps: u32 = 0;
    while input > 0 {
        if input >= 100 {
            input -= 100;
            steps += 1;
            continue;
        }

        if input >= 20 {
            input -= 20;
            steps += 1;
            continue;
        }

        if input >= 10 {
            input -= 10;
            steps += 1;
            continue;
        }

        if input >= 5 {
            input -= 5;
            steps += 1;
            continue;
        }

        if input >= 1 {
            input -= 1;
            steps += 1;
            continue;
        }

    }

    println!("{steps}");
}
