use std::fs::read_to_string;

fn main() {
    println!(
        "Frequency: {}",
        read_to_string("input.txt")
            .expect("failed to read input file")
            .lines()
            .map(|v| v.parse::<i32>().unwrap())
            .fold(0, |a, b| a + b),
    );
}
