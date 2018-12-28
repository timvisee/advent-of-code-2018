use std::collections::BTreeSet;
use std::fs::read_to_string;

fn main() {
    // Build a list for frequency duplicate checking
    let mut numbers = BTreeSet::new();

    println!(
        "Frequency: {}",
        read_to_string("input.txt")
            .expect("failed to read input file")
            .lines()
            .map(|v| v.parse::<i32>().unwrap())
            .cycle()
            .scan(0, |a, b| {
                *a += b;
                Some(*a)
            })
            .find(|x| !numbers.insert(*x))
            .unwrap(),
    );
}
