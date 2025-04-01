use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Must provide input");

    let input: Vec<u8> = input
        .split_whitespace()
        .filter_map(|input| input.parse::<u8>().ok())
        .collect();

    let ascii_constant: u8 = 96;

    let mut acc_str = String::new();

    for letter in input {
        let character: char = (letter + ascii_constant) as char;
        acc_str.push_str(&character.to_string());
    }

    println!("{}", acc_str);
}
