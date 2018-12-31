use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    // Read the input file
    let input = read_to_string("input.txt")
        .expect("failed to read input file");

    // Count line characters, totall them
    let counted: Vec<(bool, bool)> = input
        .lines()
        .map(twice_thrice)
        .collect();

    // Report the result
    println!(
        "Result: {}",
        counted
            .iter()
            .filter(|e| e.0)
            .count()
        * counted
            .iter()
            .filter(|e| e.1)
            .count()
    );
}

/// Check whether the given sequence of characters contains a character exactly twice or thrice.
fn twice_thrice(seq: &str) -> (bool, bool) {
    // Fold character counts
    let set = seq
        .chars()
        .fold(HashMap::new(), |mut set, c| {
            *set.entry(c)
                .or_insert(0) += 1;
            set
        });

    // Count and return result
    (
        set.iter()
            .any(|(_, c)| *c == 2),
        set.iter()
            .any(|(_, c)| *c == 3),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_test() {
        assert_eq!(count(""), (0, 0));
        assert_eq!(count("aa"), (1, 0));
        assert_eq!(count("aaa"), (0, 1));
        assert_eq!(count("aabbccc"), (2, 1));
        assert_eq!(count("abcabcaabbc"), (0, 1));
    }
}
