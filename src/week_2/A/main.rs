use std::io::{stdin, Read};

fn main() {
    let mut c = [0; 1];

    stdin().read_exact(&mut c).unwrap();

    let c = c[0] as char;

    let vowels = ['a', 'e', 'i', 'o', 'u'];

    if vowels.contains(&c) {
        println!("vowel");
        return;
    }

    println!("consonant");
}
