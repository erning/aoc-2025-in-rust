fn parse_input(input: &str) -> Vec<(char, Vec<u64>)> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return vec![];
    }

    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let padded: Vec<String> = lines
        .iter()
        .map(|l| format!("{:width$}", l, width = max_len))
        .collect();

    let mut problems = Vec::new();
    let mut col = 0;

    while col < max_len {
        // Skip separator columns (all spaces)
        if padded.iter().all(|line| {
            line.chars().nth(col).map(|c| c == ' ').unwrap_or(true)
        }) {
            col += 1;
            continue;
        }

        // Find the end of this problem (next all-space column or end)
        let mut end_col = col + 1;
        while end_col < max_len {
            if padded.iter().all(|line| {
                line.chars().nth(end_col).map(|c| c == ' ').unwrap_or(true)
            }) {
                break;
            }
            end_col += 1;
        }

        // Extract this problem
        let mut numbers = Vec::new();
        let mut op = '+';

        for line in &padded {
            let segment: String =
                line.chars().skip(col).take(end_col - col).collect();
            let trimmed = segment.trim();
            if trimmed == "+" || trimmed == "*" {
                op = trimmed.chars().next().unwrap();
            } else if !trimmed.is_empty() {
                if let Ok(n) = trimmed.parse::<u64>() {
                    numbers.push(n);
                }
            }
        }

        if !numbers.is_empty() {
            problems.push((op, numbers));
        }

        col = end_col;
    }

    problems
}

fn solve_problem(op: char, numbers: &[u64]) -> u64 {
    match op {
        '+' => numbers.iter().sum(),
        '*' => numbers.iter().product(),
        _ => 0,
    }
}

pub fn part_one(input: &str) -> u64 {
    let problems = parse_input(input);
    problems
        .iter()
        .map(|(op, nums)| solve_problem(*op, nums))
        .sum()
}

pub fn part_two(_input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(6);
        assert_eq!(part_one(&input), 4277556);
    }
}
