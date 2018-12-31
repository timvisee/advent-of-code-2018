extern crate rayon;
extern crate regex;
#[macro_use]
extern crate lazy_static;

use std::fs::read_to_string;

use rayon::prelude::*;
use regex::Regex;

/// The size of one dimention.
const SIZE: usize = 1000;

lazy_static! {
    /// Regex expression used for square parsing.
    static ref SQUARE_REGEX: Regex = Regex::new(
        r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$"
    ).unwrap();
}

fn main() {
    // Define the whole field
    let mut field = [[0usize; SIZE]; SIZE];

    // Parse squares from input file
    read_to_string("input.txt")
        .expect("failed to read input from file")
        .lines()
        .map(parse_square)
        .for_each(|s|
            field.iter_mut()
                .skip(s.y)
                .take(s.h)
                .for_each(|row| row
                    .iter_mut()
                    .skip(s.x)
                    .take(s.w)
                    .for_each(|c| *c += 1)
                )
        );

    // Count the cells covered by more than one square
    let covered: usize = field
        .par_iter()
        .map(|col| col
             .iter()
             .filter(|c| **c > 1)
             .count()
        )
        .sum();

    println!("Covered inches: {}", covered);
}

/// Represents a square at the given position in a grid.
#[derive(Debug, PartialEq, Eq)]
struct Square {
    /// The square number.
    n: usize,

    /// The square x coordinate, on the left.
    x: usize,

    /// The square y coordinate, on the right.
    y: usize,

    /// The square width.
    w: usize,

    /// The square height.
    h: usize,
}

/// Parse a square definition.
fn parse_square(def: &str) -> Square {
    let cap = SQUARE_REGEX
        .captures(def)
        .unwrap();

    Square {
        n: cap.get(1).unwrap().as_str().parse().unwrap(),
        x: cap.get(2).unwrap().as_str().parse().unwrap(),
        y: cap.get(3).unwrap().as_str().parse().unwrap(),
        w: cap.get(4).unwrap().as_str().parse().unwrap(),
        h: cap.get(5).unwrap().as_str().parse().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_square() {
        assert_eq!(
            parse_square("#1 @ 808,550: 12x22"),
            Square {
                n: 1,
                x: 808,
                y: 550,
                w: 12,
                h: 22,
            },
        );
        assert_eq!(
            parse_square("#0 @ 0,0: 0x0"),
            Square {
                n: 0,
                x: 0,
                y: 0,
                w: 0,
                h: 0,
            },
        );
    }
}
