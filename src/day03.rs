fn parse_input(input: &str) -> Vec<&str> {
    input.lines().filter(|line| !line.is_empty()).collect()
}

fn max_two_digit(bank: &str) -> u32 {
    let digits: Vec<u32> =
        bank.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let n = digits.len();
    let mut max_val = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            let val = digits[i] * 10 + digits[j];
            max_val = max_val.max(val);
        }
    }

    max_val
}

pub fn part_one(input: &str) -> u32 {
    parse_input(input)
        .iter()
        .map(|bank| max_two_digit(bank))
        .sum()
}

fn max_n_digits(bank: &str, n: usize) -> u64 {
    let digits: Vec<u8> = bank.chars().map(|c| c as u8 - b'0').collect();
    let len = digits.len();
    if n > len {
        return 0;
    }

    // Greedy: for each position, pick the largest available digit
    // that still leaves enough digits remaining for the rest
    let mut result = 0u64;
    let mut start = 0;

    for i in 0..n {
        let remaining_picks = n - i - 1;
        let end = len - remaining_picks;

        // Find the largest digit in range [start, end)
        let mut best_idx = start;
        for j in start..end {
            if digits[j] > digits[best_idx] {
                best_idx = j;
            }
        }

        result = result * 10 + digits[best_idx] as u64;
        start = best_idx + 1;
    }

    result
}

pub fn part_two(input: &str) -> u64 {
    parse_input(input)
        .iter()
        .map(|bank| max_n_digits(bank, 12))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(3);
        assert_eq!(part_one(&input), 357);
        assert_eq!(part_two(&input), 3121910778619);
    }
}
