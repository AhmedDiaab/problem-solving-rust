use std::io::stdin;

fn main() {
    let mut inputs = String::new();

    stdin().read_line(&mut inputs).expect("Must provide inputs");

    let sbp_values: Vec<f32> = inputs.split_whitespace().filter_map(|v| v.parse::<f32>().ok()).collect();

    if !(50 as f32 <= sbp_values[0]) || !( sbp_values[0] <= 300 as f32) || !(50 as f32 <= sbp_values[0]) || !( sbp_values[0] <= 300 as f32) {
        println!("A and B must be greater than or eq 50 and less than or eq 300");
        return;
    }

    if sbp_values[1] > sbp_values[0] {
        println!("B Must be greater than a");
        return;
    }

    let sbp = (((sbp_values[0] - sbp_values[1]) / 3 as f32) + sbp_values[1]);

    println!("{}", sbp);
}
