fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn count_adjacent_rolls(grid: &[Vec<char>], row: usize, col: usize) -> usize {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let mut count = 0;

    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let nr = row as i32 + dr;
            let nc = col as i32 + dc;
            if nr >= 0
                && nr < rows
                && nc >= 0
                && nc < cols
                && grid[nr as usize][nc as usize] == '@'
            {
                count += 1;
            }
        }
    }

    count
}

pub fn part_one(input: &str) -> usize {
    let grid = parse_input(input);
    let mut accessible = 0;

    for (row, line) in grid.iter().enumerate() {
        for (col, &cell) in line.iter().enumerate() {
            if cell == '@' && count_adjacent_rolls(&grid, row, col) < 4 {
                accessible += 1;
            }
        }
    }

    accessible
}

pub fn part_two(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(4);
        assert_eq!(part_one(&input), 13);
    }
}
