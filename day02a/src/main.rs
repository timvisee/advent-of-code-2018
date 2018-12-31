use std::collections::HashMap;
use std::fs::read_to_string;
use std::io;

fn main() -> io::Result<()> {
    // Read the input file
    let input = read_to_string("input.txt")?;

    // Count line characters, totall them
    let counted: Vec<(usize, usize)> = input
        .lines()
        .map(count)
        .collect();
    let result = counted.iter().filter(|e| e.0 > 0).count() *
        counted.iter().filter(|e| e.1 > 0).count();

    println!("Result: {}", result);

    Ok(())
}

/// Count characters in the given sequence that occur exactly twice or thrise.
fn count(seq: &str) -> (usize, usize) {
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
            .filter(|(_, c)| **c == 2)
            .count(),
        set.iter()
            .filter(|(_, c)| **c == 3)
            .count(),
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
