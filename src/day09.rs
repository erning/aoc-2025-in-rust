fn parse_input(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let nums: Vec<i64> =
                line.split(',').map(|n| n.parse().unwrap()).collect();
            (nums[0], nums[1])
        })
        .collect()
}

pub fn part_one(input: &str) -> i64 {
    let tiles = parse_input(input);
    let n = tiles.len();
    let mut max_area = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            let (x1, y1) = tiles[i];
            let (x2, y2) = tiles[j];
            // Rectangle with opposite corners (inclusive)
            let width = (x2 - x1).abs() + 1;
            let height = (y2 - y1).abs() + 1;
            let area = width * height;
            max_area = max_area.max(area);
        }
    }

    max_area
}

pub fn part_two(_input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(9);
        assert_eq!(part_one(&input), 50);
    }
}
