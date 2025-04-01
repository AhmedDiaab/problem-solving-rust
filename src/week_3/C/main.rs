use std::io::stdin;

fn main() {
    let mut reader = String::new();

    stdin().read_line(&mut reader).expect("Must provide input");

    let inputs_count: i8 = reader.trim().parse().expect("Must provide value between 1 and 100");

    if inputs_count < 1 || inputs_count > 100 {
        println!("Must provide value between 1 and 100");
        return;
    }

    
    let mut rooms_count: i8 = 0;
    
    for _ in 1..=inputs_count {
        reader.clear();
        stdin().read_line(&mut reader).expect("Must provide input");

        let room_info: Vec<i8> = reader.split_whitespace().filter_map(|i| i.parse::<i8>().ok()).collect();

        let living_persons = room_info[0];
        let room_capacity = room_info[1];

        if (living_persons < room_capacity) && (room_capacity - 2 >= living_persons) {
            rooms_count += 1;
        }
    }



    println!("{}", rooms_count);
}
