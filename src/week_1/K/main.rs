use std::io::stdin;

fn main() {
    let mut distance = String::new();

    stdin()
        .read_line(&mut distance)
        .expect("Must provide input");

    let distance: Vec<i32> = distance
        .split_whitespace()
        .filter_map(|l| l.parse::<i32>().ok())
        .collect();

    let distance = distance[0];

    if distance < 1 || distance > 1000 {
        println!("distance must be between 1 and 1000");
        return;
    }

    let mut harry_projectile = String::new();

    stdin()
        .read_line(&mut harry_projectile)
        .expect("Must provide input");

    let harry_projectile: Vec<i32> = harry_projectile
        .split_whitespace()
        .filter_map(|l| l.parse::<i32>().ok())
        .collect();

    let harry_projectile = harry_projectile[0];

    if harry_projectile < 1 || harry_projectile > 500 {
        println!("Harry projectile must be between 1 and 1000");
        return;
    }

    let mut vlordmort_projectile = String::new();

    stdin()
        .read_line(&mut vlordmort_projectile)
        .expect("Must provide input");

    let vlordmort_projectile: Vec<i32> = vlordmort_projectile
        .split_whitespace()
        .filter_map(|l| l.parse::<i32>().ok())
        .collect();

    let vlordmort_projectile = vlordmort_projectile[0];

    if vlordmort_projectile < 1 || vlordmort_projectile > 500 {
        println!("Vlordmort projectile must be between 1 and 1000");
        return;
    }

    let time = distance as f32 / (vlordmort_projectile + harry_projectile) as f32;

    let output: f32 = harry_projectile as f32 * time;

    println!("{output}");
}
