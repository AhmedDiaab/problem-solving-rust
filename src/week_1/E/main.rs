use std::io::stdin;

fn main() {
    let mut inputs = String::new();

    stdin().read_line(&mut inputs).expect("Must pass values");

    let tickets_cost: Vec<i32> = inputs
        .split_whitespace()
        .filter_map(|t| t.parse::<i32>().ok())
        .collect();

    if tickets_cost[1] % 2 != 0 {
        println!("Bus ticket price is an even number");
        return;
    }

    let sum: i32 = tickets_cost[0] + (tickets_cost[1] / 2);

    

    println!("{}", sum);

}
