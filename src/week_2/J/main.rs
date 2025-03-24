use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Must provide value");

    let input: Vec<u32> = input
        .split_whitespace()
        .filter_map(|n| n.parse::<u32>().ok())
        .collect();

    let takahashi = input[0];
    let aoki = input[1];
    let start = input[2]; // 0 takahashi | 1 aoki


    let mut winner = "";

    if start == 0 { // takahashi
        if aoki >= takahashi {
            winner = "Aoki";
        }
        else {
            winner = "Takahashi";
        }
    } else if start == 1 {
        if takahashi >= aoki {
            winner = "Takahashi";
        } else {
            winner = "Aoki";
        }
    }

    println!("{}", winner);
}
