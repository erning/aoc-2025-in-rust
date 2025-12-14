fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let ranges: Vec<(u64, u64)> = parts[0]
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut nums = line.split('-');
            let start = nums.next().unwrap().parse().unwrap();
            let end = nums.next().unwrap().parse().unwrap();
            (start, end)
        })
        .collect();

    let ids: Vec<u64> = parts[1]
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();

    (ranges, ids)
}

fn is_fresh(id: u64, ranges: &[(u64, u64)]) -> bool {
    ranges.iter().any(|&(start, end)| id >= start && id <= end)
}

pub fn part_one(input: &str) -> usize {
    let (ranges, ids) = parse_input(input);
    ids.iter().filter(|&&id| is_fresh(id, &ranges)).count()
}

pub fn part_two(input: &str) -> u64 {
    let (ranges, _) = parse_input(input);

    // Merge overlapping ranges
    let mut sorted: Vec<(u64, u64)> = ranges;
    sorted.sort_by_key(|r| r.0);

    let mut merged: Vec<(u64, u64)> = Vec::new();
    for (start, end) in sorted {
        if let Some(last) = merged.last_mut() {
            if start <= last.1 + 1 {
                last.1 = last.1.max(end);
            } else {
                merged.push((start, end));
            }
        } else {
            merged.push((start, end));
        }
    }

    // Count total IDs
    merged.iter().map(|(s, e)| e - s + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(5);
        assert_eq!(part_one(&input), 3);
        assert_eq!(part_two(&input), 14);
    }
}
