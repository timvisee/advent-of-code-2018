extern crate itertools;

use std::fs::read_to_string;

use itertools::Itertools;

fn main() {
    // Read the input file
    let input = read_to_string("input.txt")
        .expect("failed to read input file");

    // Count line characters, totall them
    println!(
        "Characters: {}",
        input
            .lines()
            .tuple_combinations()
            .filter(|(a, b)|
                // Keep items that have one differing character
                a.chars()
                    .zip(b.chars())
                    .filter(|(a, b)| a != b)
                    .count() == 1
            )
            .map(|(a, b)|
                a.chars()
                    .zip(b.chars())
                    .filter(|(a, b)| a == b)
                    .map(|(a, _)| a)
                    .collect::<String>()
            )
            .next()
            .expect("no combination found")
    );
}
