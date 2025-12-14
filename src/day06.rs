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

fn parse_input_v2(input: &str) -> Vec<(char, Vec<u64>)> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return vec![];
    }

    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let padded: Vec<Vec<char>> = lines
        .iter()
        .map(|l| {
            let mut chars: Vec<char> = l.chars().collect();
            chars.resize(max_len, ' ');
            chars
        })
        .collect();

    let num_rows = padded.len();
    let mut problems = Vec::new();
    let mut col = max_len;

    while col > 0 {
        col -= 1;

        // Skip separator columns (all spaces)
        if padded.iter().all(|line| line[col] == ' ') {
            continue;
        }

        // Find the start of this problem (previous all-space column or start)
        let mut start_col = col;
        while start_col > 0 {
            if padded.iter().all(|line| line[start_col - 1] == ' ') {
                break;
            }
            start_col -= 1;
        }

        // Get the operator from the last row
        let op = padded[num_rows - 1][start_col..=col]
            .iter()
            .find(|&&c| c == '+' || c == '*')
            .copied()
            .unwrap_or('+');

        // Read numbers column by column from right to left
        let mut numbers = Vec::new();
        for c in (start_col..=col).rev() {
            let num_str: String = padded
                .iter()
                .take(num_rows - 1)
                .filter_map(|row| {
                    let ch = row[c];
                    if ch.is_ascii_digit() {
                        Some(ch)
                    } else {
                        None
                    }
                })
                .collect();
            if !num_str.is_empty() {
                numbers.push(num_str.parse::<u64>().unwrap());
            }
        }

        if !numbers.is_empty() {
            problems.push((op, numbers));
        }

        col = start_col;
    }

    problems
}

pub fn part_two(input: &str) -> u64 {
    let problems = parse_input_v2(input);
    problems
        .iter()
        .map(|(op, nums)| solve_problem(*op, nums))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(6);
        assert_eq!(part_one(&input), 4277556);
        assert_eq!(part_two(&input), 3263827);
    }
}
