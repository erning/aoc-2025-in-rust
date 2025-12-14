use std::collections::HashSet;

fn parse_input(input: &str) -> (Vec<Vec<char>>, usize) {
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    // Find S position (column)
    let start_col = grid[0].iter().position(|&c| c == 'S').unwrap();

    (grid, start_col)
}

pub fn part_one(input: &str) -> usize {
    let (grid, start_col) = parse_input(input);
    let rows = grid.len();
    let cols = grid[0].len();

    // Track active beams as (row, col) positions
    let mut beams: HashSet<(usize, usize)> = HashSet::new();
    beams.insert((0, start_col));

    let mut split_count = 0;

    loop {
        let mut new_beams: HashSet<(usize, usize)> = HashSet::new();

        for &(row, col) in &beams {
            let next_row = row + 1;
            if next_row >= rows {
                continue; // Beam exits the manifold
            }

            let cell = grid[next_row][col];
            if cell == '^' {
                // Split: emit beams to left and right
                split_count += 1;
                if col > 0 {
                    new_beams.insert((next_row, col - 1));
                }
                if col + 1 < cols {
                    new_beams.insert((next_row, col + 1));
                }
            } else {
                // Continue downward
                new_beams.insert((next_row, col));
            }
        }

        if new_beams.is_empty() {
            break;
        }
        beams = new_beams;
    }

    split_count
}

use std::collections::HashMap;

pub fn part_two(input: &str) -> u64 {
    let (grid, start_col) = parse_input(input);
    let rows = grid.len();
    let cols = grid[0].len();

    // Track beams with timeline counts: position -> number of timelines at that position
    let mut beams: HashMap<(usize, usize), u64> = HashMap::new();
    beams.insert((0, start_col), 1);

    let mut total_timelines = 0u64;

    loop {
        let mut new_beams: HashMap<(usize, usize), u64> = HashMap::new();

        for (&(row, col), &count) in &beams {
            let next_row = row + 1;
            if next_row >= rows {
                // Beam exits - these timelines are complete
                total_timelines += count;
                continue;
            }

            let cell = grid[next_row][col];
            if cell == '^' {
                // Split: each timeline becomes two timelines
                if col > 0 {
                    *new_beams.entry((next_row, col - 1)).or_insert(0) +=
                        count;
                }
                if col + 1 < cols {
                    *new_beams.entry((next_row, col + 1)).or_insert(0) +=
                        count;
                }
            } else {
                // Continue downward
                *new_beams.entry((next_row, col)).or_insert(0) += count;
            }
        }

        if new_beams.is_empty() {
            break;
        }
        beams = new_beams;
    }

    total_timelines
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(7);
        assert_eq!(part_one(&input), 21);
        assert_eq!(part_two(&input), 40);
    }
}
