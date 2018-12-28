use std::fs::read_to_string;

fn main() {
    // Read the input
    let input = read_to_string("input.txt").expect("failed to read input file");

    println!(
        "Frequency: {}",
        input.lines()
            .map(|v| v.parse::<i32>().unwrap())
            .fold(0, |a, b| a + b),
    );
}
