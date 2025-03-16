use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Must provide value");

    let input: Vec<i32> = input.split_whitespace().filter_map(|v| v.parse::<i32>().ok()).collect();

    let mut input = input[0];

    let mut steps = 0;

    while input > 0 {
        if input >= 5 {
            steps += 1;
            input -= 5;
            continue;
        }
        steps += 1;
        input -= input;
    }

    println!("{steps}");
}
