use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Must provide input");

    let operations_count: u8 = input.trim().parse().unwrap();

    if operations_count > 150 {
        println!("Operations is between 1 and 150");
        return;
    }

    let mut sum = 0;
    for _ in 1..=operations_count {
        input.clear();

        stdin().read_line(&mut input).expect("Must provide input");

        if input.trim().contains("++") {
            sum += 1;
        } else if input.trim().contains("--") {
            sum -= 1;
        }
    }

    println!("{}", sum);
}
