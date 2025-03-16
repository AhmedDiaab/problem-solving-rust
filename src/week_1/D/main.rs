use std::io::stdin;

fn main() {
    let mut line = String::new();

    stdin().read_line(&mut line).expect("Must pass values");

    let faces: Vec<i32> = line
        .split_whitespace()
        .filter_map(|f| f.parse::<i32>().ok())
        .collect();

    
    let mut sum: i32 = 0;

    if faces.iter().any(|&f| !(1..=6).contains(&f)) {
        println!("Items include out of bound value");
        return;
    }

    for face in &faces {
        sum += (7 - face);
    }

    println!("{}", sum);
}
