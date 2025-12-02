//! Day 1: Report Repair
//!
//! ## Problem Description
//!
//! Part 1: Find two numbers in the expense report that sum to 2020 and return their product.
//! Part 2: Find three numbers in the expense report that sum to 2020 and return their product.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Converts the multiline string input into a vector of integers.
//!
//! **Part 1 Strategy**: Uses a brute-force nested loop approach:
//! - Iterates through all pairs of numbers using two nested loops
//! - Outer loop: takes each number `a` from the start to second-to-last
//! - Inner loop: takes each number `b` from current `a` position to end
//! - Checks if `a + b == 2020`
//! - Returns `a * b` immediately when found
//!
//! **Part 2 Strategy**: Extends the same approach to three numbers:
//! - Uses three nested loops with similar indexing pattern
//! - Outer loop: number `a` from start to third-to-last
//! - Middle loop: number `b` from current `a` position to second-to-last
//! - Inner loop: number `c` from current `b` position to end
//! - Checks if `a + b + c == 2020`
//! - Returns `a * b * c` immediately when found
//!
//! **Complexity**: O(n²) for part 1, O(n³) for part 2 where n is the number of entries.
//! **Optimization Note**: Could be improved with hash sets for O(n) part 1 and O(n²) part 2.

fn parse_input(input: &str) -> Vec<i32> {
    input.trim().lines().map(|s| s.parse().unwrap()).collect()
}

pub fn part_one(input: &str) -> i32 {
    let numbers = parse_input(input);
    let n = numbers.len();
    for (i, a) in numbers.iter().take(n - 1).enumerate() {
        for b in numbers.iter().skip(i) {
            if a + b == 2020 {
                return a * b;
            }
        }
    }
    panic!()
}

pub fn part_two(input: &str) -> i32 {
    let numbers = parse_input(input);
    let n = numbers.len();
    for (i, a) in numbers.iter().enumerate().take(n - 2) {
        for (j, b) in numbers.iter().enumerate().take(n - 1).skip(i) {
            for c in numbers.iter().skip(j) {
                if a + b + c == 2020 {
                    return a * b * c;
                }
            }
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(1);
        assert_eq!(part_one(&input), 514579);
        assert_eq!(part_two(&input), 241861950);
    }
}
