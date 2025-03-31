use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Must provide input");

    let input: u32 = input
        .trim()
        .parse::<u32>()
        .ok()
        .expect("Must provide a number between 1 and 100");

    if input > 100 {
        println!("Must provide a number between 1 and 100");
        return;
    }

    let mut sum: u32 = 0;
    for i in 1..=input {
        sum += i;
    }

    println!("{}", sum);
}
