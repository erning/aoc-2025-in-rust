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

pub fn part_two(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(3);
        assert_eq!(part_one(&input), 357);
    }
}
