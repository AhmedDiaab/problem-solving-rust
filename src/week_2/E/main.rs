use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Must provide input");

    let input: Vec<u128> = input
        .split_whitespace()
        .filter_map(|d| d.parse::<u128>().ok())
        .collect();

    let n = input[0] as f64;
    let m = input[1] as f64;
    let a = input[2] as f64;

    let flagstone_area_width = (n / a).ceil();
    let flagstone_area_height = (m / a).ceil();
    let flagstone_area = flagstone_area_width * flagstone_area_height;

    println!("{}", flagstone_area);
}
