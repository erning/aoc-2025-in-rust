fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .trim()
        .split(',')
        .map(|range| {
            let mut parts = range.split('-');
            let start = parts.next().unwrap().parse().unwrap();
            let end = parts.next().unwrap().parse().unwrap();
            (start, end)
        })
        .collect()
}

// Generate all doubled numbers within a range
fn find_doubled_in_range(start: u64, end: u64) -> Vec<u64> {
    let mut result = Vec::new();

    // Determine the digit lengths we need to check
    let start_digits = start.to_string().len();
    let end_digits = end.to_string().len();

    for total_digits in start_digits..=end_digits {
        if !total_digits.is_multiple_of(2) {
            continue;
        }

        let half_digits = total_digits / 2;
        let min_half = 10u64.pow((half_digits - 1) as u32);
        let max_half = 10u64.pow(half_digits as u32) - 1;

        for half in min_half..=max_half {
            let doubled = half * 10u64.pow(half_digits as u32) + half;
            if doubled >= start && doubled <= end {
                result.push(doubled);
            }
        }
    }

    result
}

pub fn part_one(input: &str) -> u64 {
    let ranges = parse_input(input);
    let mut sum = 0;

    for (start, end) in ranges {
        for doubled in find_doubled_in_range(start, end) {
            sum += doubled;
        }
    }

    sum
}

// Generate all repeated-pattern numbers within a range (pattern repeated at least twice)
fn find_repeated_in_range(start: u64, end: u64) -> Vec<u64> {
    let mut result = Vec::new();

    let start_digits = start.to_string().len();
    let end_digits = end.to_string().len();

    for total_digits in start_digits..=end_digits {
        // Try each possible pattern length (1 to total_digits/2)
        for pattern_len in 1..=total_digits / 2 {
            if !total_digits.is_multiple_of(pattern_len) {
                continue;
            }

            let repeat_count = total_digits / pattern_len;
            if repeat_count < 2 {
                continue;
            }

            // Generate all patterns of this length
            let min_pattern = if pattern_len == 1 {
                1
            } else {
                10u64.pow((pattern_len - 1) as u32)
            };
            let max_pattern = 10u64.pow(pattern_len as u32) - 1;

            for pattern in min_pattern..=max_pattern {
                // Build the repeated number
                let mut num = 0u64;
                let multiplier = 10u64.pow(pattern_len as u32);
                for _ in 0..repeat_count {
                    num = num * multiplier + pattern;
                }

                if num >= start && num <= end {
                    result.push(num);
                }
            }
        }
    }

    // Remove duplicates and sort
    result.sort();
    result.dedup();
    result
}

pub fn part_two(input: &str) -> u64 {
    let ranges = parse_input(input);
    let mut sum = 0;

    for (start, end) in ranges {
        for repeated in find_repeated_in_range(start, end) {
            sum += repeated;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn test_find_doubled() {
        assert_eq!(find_doubled_in_range(11, 22), vec![11, 22]);
        assert_eq!(find_doubled_in_range(95, 115), vec![99]);
        assert_eq!(find_doubled_in_range(998, 1012), vec![1010]);
    }

    #[test]
    fn example() {
        let input = read_example(2);
        assert_eq!(part_one(&input), 1227775554);
        assert_eq!(part_two(&input), 4174379265);
    }
}
