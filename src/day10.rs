fn parse_line(line: &str) -> (Vec<bool>, Vec<Vec<usize>>, Vec<i64>) {
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
            break;
        } else {
            i += 1;
        }
    }

    // Parse joltage requirements {x,y,z}
    let brace_start = line.find('{').unwrap();
    let brace_end = line.find('}').unwrap();
    let joltage: Vec<i64> = line[brace_start + 1..brace_end]
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();

    (lights, buttons, joltage)
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
                let pivot_row_copy: Vec<bool> = matrix[pivot_row].clone();
                for (c, val) in matrix[row].iter_mut().enumerate().take(n_buttons + 1) {
                    *val = *val != pivot_row_copy[c];
                }
            }
        }

        pivot_row += 1;
    }

    // Check for inconsistency
    for row in matrix.iter().take(n_lights).skip(pivot_row) {
        if row[n_buttons] {
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
            let (target, buttons, _) = parse_line(line);
            solve_machine(&target, &buttons).unwrap_or(0)
        })
        .sum()
}

// Part 2: Integer linear programming - each button press increments counters
// We need to find non-negative integers x_i such that:
// sum(x_i * a_ij) = b_j for each counter j
// and minimize sum(x_i)
fn solve_joltage(target: &[i64], buttons: &[Vec<usize>]) -> i64 {
    let n_counters = target.len();
    let n_buttons = buttons.len();

    // Build coefficient matrix: A[j][i] = 1 if button i affects counter j
    let mut a: Vec<Vec<i64>> = vec![vec![0; n_buttons]; n_counters];
    for (i, button) in buttons.iter().enumerate() {
        for &j in button {
            if j < n_counters {
                a[j][i] = 1;
            }
        }
    }

    // Use brute force with bounded search for small inputs
    // For each button, max presses needed is max(target) since each button adds at most 1
    let max_press = *target.iter().max().unwrap_or(&0) as usize;

    // Use iterative deepening / BFS-like approach
    // Generate all combinations up to a certain total presses
    solve_ilp(&a, target, n_buttons, max_press)
}

fn solve_ilp(
    a: &[Vec<i64>],
    target: &[i64],
    n_buttons: usize,
    max_total: usize,
) -> i64 {
    // Try to find solution using Gaussian elimination on integers
    if let Some(result) = solve_nonneg_integer(a, target, n_buttons) {
        return result;
    }

    // Fallback: brute force for small cases
    if n_buttons <= 10 && max_total <= 20 {
        for total in 0..=max_total {
            if let Some(presses) =
                find_combination(a, target, n_buttons, total)
            {
                return presses as i64;
            }
        }
    }

    0
}

fn solve_nonneg_integer(
    a: &[Vec<i64>],
    target: &[i64],
    n_buttons: usize,
) -> Option<i64> {
    let n_counters = target.len();

    // Build augmented matrix [A | b]
    let mut matrix: Vec<Vec<i64>> = vec![vec![0; n_buttons + 1]; n_counters];
    for j in 0..n_counters {
        for i in 0..n_buttons {
            matrix[j][i] = a[j][i];
        }
        matrix[j][n_buttons] = target[j];
    }

    // Gaussian elimination with integer operations (using GCD)
    let mut pivot_cols: Vec<usize> = Vec::new();
    let mut pivot_row = 0;

    for col in 0..n_buttons {
        // Find non-zero pivot
        let mut found = false;
        for row in pivot_row..n_counters {
            if matrix[row][col] != 0 {
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
        for row in 0..n_counters {
            if row != pivot_row && matrix[row][col] != 0 {
                let g =
                    gcd(matrix[pivot_row][col].abs(), matrix[row][col].abs());
                let mult_pivot = matrix[row][col] / g;
                let mult_row = matrix[pivot_row][col] / g;

                let pivot_row_copy: Vec<i64> = matrix[pivot_row].clone();
                for (c, val) in matrix[row].iter_mut().enumerate().take(n_buttons + 1) {
                    *val = *val * mult_row - pivot_row_copy[c] * mult_pivot;
                }
            }
        }

        pivot_row += 1;
    }

    // Check for inconsistency
    for row in matrix.iter().take(n_counters).skip(pivot_row) {
        if row[n_buttons] != 0 {
            return None;
        }
    }

    // Free variables
    let free_cols: Vec<usize> =
        (0..n_buttons).filter(|c| !pivot_cols.contains(c)).collect();

    // Find minimum sum solution by trying different free variable values
    let mut min_presses = i64::MAX;
    // Max search should be at least max(target)
    let max_search = (*target.iter().max().unwrap_or(&50)).max(50);

    search_solution(
        &matrix,
        &pivot_cols,
        &free_cols,
        n_buttons,
        0,
        &mut vec![0i64; n_buttons],
        max_search,
        &mut min_presses,
        0, // current_sum starts at 0
    );

    if min_presses == i64::MAX {
        None
    } else {
        Some(min_presses)
    }
}

#[allow(clippy::too_many_arguments)]
fn search_solution(
    matrix: &[Vec<i64>],
    pivot_cols: &[usize],
    free_cols: &[usize],
    n_buttons: usize,
    free_idx: usize,
    solution: &mut Vec<i64>,
    max_val: i64,
    min_presses: &mut i64,
    current_sum: i64,
) {
    // Pruning: if current sum of free variables already >= best solution, skip
    if current_sum >= *min_presses {
        return;
    }

    if free_idx == free_cols.len() {
        // All free variables set, compute pivot variables
        let mut sol = solution.clone();
        let mut pivot_sum: i64 = 0;

        for (row, &col) in pivot_cols.iter().enumerate().rev() {
            let mut sum = matrix[row][n_buttons];
            for c in (col + 1)..n_buttons {
                sum -= matrix[row][c] * sol[c];
            }

            if matrix[row][col] == 0 || sum % matrix[row][col] != 0 {
                return;
            }

            let val = sum / matrix[row][col];
            if val < 0 {
                return; // Early exit if negative
            }
            sol[col] = val;
            pivot_sum += val;
            
            // Early pruning during back-substitution
            if current_sum + pivot_sum >= *min_presses {
                return;
            }
        }

        let total = current_sum + pivot_sum;
        *min_presses = (*min_presses).min(total);
        return;
    }

    let col = free_cols[free_idx];
    // Limit max_val based on remaining budget
    let remaining_budget = *min_presses - current_sum - 1;
    let effective_max = if remaining_budget >= 0 {
        max_val.min(remaining_budget)
    } else {
        return;
    };

    for val in 0..=effective_max {
        solution[col] = val;
        search_solution(
            matrix,
            pivot_cols,
            free_cols,
            n_buttons,
            free_idx + 1,
            solution,
            max_val,
            min_presses,
            current_sum + val,
        );
    }
    solution[col] = 0;
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn find_combination(
    a: &[Vec<i64>],
    target: &[i64],
    n_buttons: usize,
    total: usize,
) -> Option<usize> {
    // Try to find combination of exactly `total` presses
    let n_counters = target.len();

    fn backtrack(
        a: &[Vec<i64>],
        target: &[i64],
        state: &mut Vec<i64>,
        button: usize,
        remaining: usize,
        n_buttons: usize,
        n_counters: usize,
    ) -> bool {
        if button == n_buttons {
            return remaining == 0 && state == target;
        }

        // Pruning: check if any counter is already exceeded
        for j in 0..n_counters {
            if state[j] > target[j] {
                return false;
            }
        }

        // Try 0 to remaining presses for this button
        for presses in 0..=remaining {
            for j in 0..n_counters {
                state[j] += a[j][button] * presses as i64;
            }

            if backtrack(
                a,
                target,
                state,
                button + 1,
                remaining - presses,
                n_buttons,
                n_counters,
            ) {
                return true;
            }

            for j in 0..n_counters {
                state[j] -= a[j][button] * presses as i64;
            }
        }

        false
    }

    let mut state = vec![0i64; n_counters];
    if backtrack(a, target, &mut state, 0, total, n_buttons, n_counters) {
        Some(total)
    } else {
        None
    }
}

pub fn part_two(input: &str) -> i64 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (_, buttons, joltage) = parse_line(line);
            solve_joltage(&joltage, &buttons)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(10);
        assert_eq!(part_one(&input), 7);
        assert_eq!(part_two(&input), 33);
    }
}
