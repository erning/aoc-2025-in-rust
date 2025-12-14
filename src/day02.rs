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

pub fn part_two(_input: &str) -> u64 {
    0 // Part 2 not yet available
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
    }
}
