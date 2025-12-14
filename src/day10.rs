fn parse_line(line: &str) -> (Vec<bool>, Vec<Vec<usize>>) {
    // Parse indicator lights [.##.]
    let bracket_start = line.find('[').unwrap();
    let bracket_end = line.find(']').unwrap();
    let lights: Vec<bool> = line[bracket_start + 1..bracket_end]
        .chars()
        .map(|c| c == '#')
        .collect();

    // Parse button wiring schematics (x,y,z)
    let mut buttons: Vec<Vec<usize>> = Vec::new();
    let mut i = bracket_end + 1;
    let chars: Vec<char> = line.chars().collect();

    while i < chars.len() {
        if chars[i] == '(' {
            let paren_end = line[i..].find(')').unwrap() + i;
            let content = &line[i + 1..paren_end];
            let indices: Vec<usize> = content
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();
            buttons.push(indices);
            i = paren_end + 1;
        } else if chars[i] == '{' {
            break; // Stop at joltage requirements
        } else {
            i += 1;
        }
    }

    (lights, buttons)
}

fn solve_machine(target: &[bool], buttons: &[Vec<usize>]) -> Option<usize> {
    let n_lights = target.len();
    let n_buttons = buttons.len();

    // Try all combinations (brute force for small inputs)
    // For larger inputs, use Gaussian elimination on GF(2)
    if n_buttons <= 20 {
        let mut min_presses = usize::MAX;

        for mask in 0..(1u32 << n_buttons) {
            let mut state = vec![false; n_lights];
            let mut presses = 0;

            for (i, button) in buttons.iter().enumerate() {
                if mask & (1 << i) != 0 {
                    presses += 1;
                    for &idx in button {
                        if idx < n_lights {
                            state[idx] = !state[idx];
                        }
                    }
                }
            }

            if state == target {
                min_presses = min_presses.min(presses);
            }
        }

        if min_presses == usize::MAX {
            None
        } else {
            Some(min_presses)
        }
    } else {
        // Use Gaussian elimination for larger inputs
        solve_gaussian(target, buttons)
    }
}

fn solve_gaussian(target: &[bool], buttons: &[Vec<usize>]) -> Option<usize> {
    let n_lights = target.len();
    let n_buttons = buttons.len();

    // Build augmented matrix [A | b] over GF(2)
    // Each row is a light, each column is a button, last column is target
    let mut matrix: Vec<Vec<bool>> =
        vec![vec![false; n_buttons + 1]; n_lights];

    for (col, button) in buttons.iter().enumerate() {
        for &idx in button {
            if idx < n_lights {
                matrix[idx][col] = true;
            }
        }
    }

    for (row, &t) in target.iter().enumerate() {
        matrix[row][n_buttons] = t;
    }

    // Gaussian elimination
    let mut pivot_cols: Vec<usize> = Vec::new();
    let mut pivot_row = 0;

    for col in 0..n_buttons {
        // Find pivot
        let mut found = false;
        for row in pivot_row..n_lights {
            if matrix[row][col] {
                matrix.swap(pivot_row, row);
                found = true;
                break;
            }
        }

        if !found {
            continue;
        }

        pivot_cols.push(col);

        // Eliminate
        for row in 0..n_lights {
            if row != pivot_row && matrix[row][col] {
                for c in 0..=n_buttons {
                    matrix[row][c] = matrix[row][c] != matrix[pivot_row][c];
                }
            }
        }

        pivot_row += 1;
    }

    // Check for inconsistency
    for row in pivot_row..n_lights {
        if matrix[row][n_buttons] {
            return None; // No solution
        }
    }

    // Free variables are columns not in pivot_cols
    let free_cols: Vec<usize> =
        (0..n_buttons).filter(|c| !pivot_cols.contains(c)).collect();

    // Try all combinations of free variables to minimize presses
    let n_free = free_cols.len();
    let mut min_presses = usize::MAX;

    for free_mask in 0..(1u32 << n_free) {
        let mut solution = vec![false; n_buttons];

        // Set free variables
        for (i, &col) in free_cols.iter().enumerate() {
            solution[col] = (free_mask & (1 << i)) != 0;
        }

        // Back-substitute for pivot variables
        for (row, &col) in pivot_cols.iter().enumerate().rev() {
            let mut val = matrix[row][n_buttons];
            for c in (col + 1)..n_buttons {
                if matrix[row][c] {
                    val = val != solution[c];
                }
            }
            solution[col] = val;
        }

        let presses: usize = solution.iter().filter(|&&x| x).count();
        min_presses = min_presses.min(presses);
    }

    Some(min_presses)
}

pub fn part_one(input: &str) -> usize {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (target, buttons) = parse_line(line);
            solve_machine(&target, &buttons).unwrap_or(0)
        })
        .sum()
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
        let input = read_example(10);
        assert_eq!(part_one(&input), 7);
    }
}
