use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Must pass values");

    let input: Vec<u16> = input.split_whitespace().filter_map(|n| n.parse::<u16>().ok()).collect();


    let ball_speed = input[0]; // V
    let start_invisibility = input[1]; // T
    let end_invisibility = input[2]; // S
    let max_invisibility_distance = input[3]; // D, from takahashi, start of distance that can aoki hit


    let start_distance = ball_speed * start_invisibility;
    let end_distance = ball_speed * end_invisibility;

    if start_distance > max_invisibility_distance || end_distance < max_invisibility_distance {
        println!("Yes");
        return;
    }
    println!("No");

}