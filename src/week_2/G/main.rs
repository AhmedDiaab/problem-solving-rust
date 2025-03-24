use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Must provide input");

    let input: Vec<u8> = input
        .split_whitespace()
        .filter_map(|n| n.parse::<u8>().ok())
        .collect();

    let a = input[0];
    let b = input[1];

    // let date_str = "2018-{a}-{b}"
    //     .replace("{a}", &a.to_string())
    //     .replace("{b}", &b.to_string());

    // let parsed_date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d");

    // match parsed_date {
    //     Ok(_) => {}
    //     Err(e) => {
    //         println!("Error parsing date: {}", e);
    //         return;
    //     }
    // }

    if b >= a {
        println!("{}", a);
        return;
    }
    
    println!("{}", a - 1);
}
