use std::collections::HashSet;

type Shape = Vec<(i32, i32)>; // List of (row, col) offsets

fn parse_input(input: &str) -> (Vec<Shape>, Vec<(usize, usize, Vec<usize>)>) {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let mut shapes: Vec<Shape> = Vec::new();
    let mut regions: Vec<(usize, usize, Vec<usize>)> = Vec::new();

    // Parse shapes and regions
    for part in &parts {
        let lines: Vec<&str> = part.lines().collect();
        if lines.is_empty() {
            continue;
        }

        // Check if this is a shape definition (starts with digit and colon)
        if lines[0].chars().next().map_or(false, |c| c.is_ascii_digit())
            && lines[0].contains(':')
            && !lines[0].contains('x')
        {
            let mut shape = Vec::new();
            for (row, line) in lines[1..].iter().enumerate() {
                for (col, ch) in line.chars().enumerate() {
                    if ch == '#' {
                        shape.push((row as i32, col as i32));
                    }
                }
            }
            shapes.push(shape);
        } else {
            // This part contains regions
            for line in lines {
                if line.contains('x') && line.contains(':') {
                    let parts: Vec<&str> = line.split(':').collect();
                    let size_part = parts[0].trim();
                    let counts_part = parts[1].trim();

                    let dimensions: Vec<usize> = size_part
                        .split('x')
                        .map(|s| s.parse().unwrap())
                        .collect();
                    let width = dimensions[0];
                    let height = dimensions[1];

                    let counts: Vec<usize> = counts_part
                        .split_whitespace()
                        .map(|s| s.parse().unwrap())
                        .collect();

                    regions.push((width, height, counts));
                }
            }
        }
    }

    (shapes, regions)
}

// Generate all rotations and reflections of a shape
fn get_orientations(shape: &Shape) -> Vec<Shape> {
    let mut orientations: HashSet<Vec<(i32, i32)>> = HashSet::new();

    let mut current = shape.clone();
    for _ in 0..4 {
        // Original
        orientations.insert(normalize(&current));
        // Flip horizontal
        let flipped: Shape = current.iter().map(|&(r, c)| (r, -c)).collect();
        orientations.insert(normalize(&flipped));
        // Rotate 90 degrees: (r, c) -> (c, -r)
        current = current.iter().map(|&(r, c)| (c, -r)).collect();
    }

    orientations.into_iter().collect()
}

// Normalize shape so that minimum row and col are 0
fn normalize(shape: &Shape) -> Shape {
    let min_r = shape.iter().map(|&(r, _)| r).min().unwrap_or(0);
    let min_c = shape.iter().map(|&(_, c)| c).min().unwrap_or(0);
    let mut result: Shape = shape.iter().map(|&(r, c)| (r - min_r, c - min_c)).collect();
    result.sort();
    result
}

// Check if a shape can be placed at (row, col) on the grid
fn can_place(grid: &[Vec<bool>], shape: &Shape, row: i32, col: i32, height: usize, width: usize) -> bool {
    for &(dr, dc) in shape {
        let r = row + dr;
        let c = col + dc;
        if r < 0 || c < 0 || r >= height as i32 || c >= width as i32 {
            return false;
        }
        if grid[r as usize][c as usize] {
            return false; // Cell is already occupied
        }
    }
    true
}

// Place a shape on the grid
fn place(grid: &mut [Vec<bool>], shape: &Shape, row: i32, col: i32) {
    for &(dr, dc) in shape {
        grid[(row + dr) as usize][(col + dc) as usize] = true;
    }
}

// Remove a shape from the grid
fn unplace(grid: &mut [Vec<bool>], shape: &Shape, row: i32, col: i32) {
    for &(dr, dc) in shape {
        grid[(row + dr) as usize][(col + dc) as usize] = false;
    }
}

// Solve using backtracking with first-empty-cell optimization
// Key insight: when placing a piece, we MUST cover the first empty cell
// Unless no piece can cover it, in which case we mark it as permanently empty
fn solve(
    grid: &mut Vec<Vec<bool>>,
    width: usize,
    height: usize,
    pieces: &[Vec<Shape>], // All orientations for each piece
    remaining_pieces: &mut Vec<usize>, // Indices of pieces not yet placed
    empty_budget: usize, // How many cells can remain empty
) -> bool {
    if remaining_pieces.is_empty() {
        return true; // All pieces placed
    }

    // Find first empty cell
    let mut first_empty = None;
    'outer: for r in 0..height {
        for c in 0..width {
            if !grid[r][c] {
                first_empty = Some((r, c));
                break 'outer;
            }
        }
    }

    let (start_row, start_col) = match first_empty {
        Some(pos) => pos,
        None => return remaining_pieces.is_empty(),
    };

    // Try each remaining piece to cover the first empty cell
    let mut any_placed = false;
    for i in 0..remaining_pieces.len() {
        let piece_idx = remaining_pieces[i];
        let orientations = &pieces[piece_idx];

        // Try all orientations that can cover the first empty cell
        for orientation in orientations {
            for &(dr, dc) in orientation {
                let row = start_row as i32 - dr;
                let col = start_col as i32 - dc;

                if can_place(grid, orientation, row, col, height, width) {
                    any_placed = true;
                    place(grid, orientation, row, col);
                    remaining_pieces.remove(i);

                    if solve(grid, width, height, pieces, remaining_pieces, empty_budget) {
                        return true;
                    }

                    remaining_pieces.insert(i, piece_idx);
                    unplace(grid, orientation, row, col);
                }
            }
        }
    }

    // If no piece can cover this cell and we have empty budget, skip this cell
    if !any_placed && empty_budget > 0 {
        grid[start_row][start_col] = true; // Mark as "used" (will be empty in final)
        if solve(grid, width, height, pieces, remaining_pieces, empty_budget - 1) {
            return true;
        }
        grid[start_row][start_col] = false;
    }

    false
}

fn can_fit_precomputed(all_orientations: &[Vec<Shape>], width: usize, height: usize, counts: &[usize]) -> bool {
    // Collect all pieces to place with their orientations (using pre-computed orientations)
    let mut pieces: Vec<Vec<Shape>> = Vec::new();
    let mut piece_sizes: Vec<usize> = Vec::new();

    for (shape_idx, &count) in counts.iter().enumerate() {
        if count > 0 && shape_idx < all_orientations.len() {
            let orientations = &all_orientations[shape_idx];
            let size = orientations[0].len();
            for _ in 0..count {
                pieces.push(orientations.clone());
                piece_sizes.push(size);
            }
        }
    }

    // Check if total cells exceeds grid
    let total_cells: usize = piece_sizes.iter().sum();
    if total_cells > width * height {
        return false;
    }

    // Sort pieces by size (largest first for better pruning)
    // Create indices sorted by piece size
    let mut indices: Vec<usize> = (0..pieces.len()).collect();
    indices.sort_by_key(|&i| std::cmp::Reverse(piece_sizes[i]));

    // Reorder pieces
    let sorted_pieces: Vec<Vec<Shape>> = indices.iter().map(|&i| pieces[i].clone()).collect();

    let mut grid = vec![vec![false; width]; height];
    let mut remaining: Vec<usize> = (0..sorted_pieces.len()).collect();
    let empty_budget = width * height - total_cells;
    solve(&mut grid, width, height, &sorted_pieces, &mut remaining, empty_budget)
}

pub fn part_one(input: &str) -> usize {
    let (shapes, regions) = parse_input(input);
    
    // Pre-compute all orientations for all shapes
    let all_orientations: Vec<Vec<Shape>> = shapes.iter()
        .map(|shape| get_orientations(shape))
        .collect();
    
    regions
        .iter()
        .filter(|(width, height, counts)| can_fit_precomputed(&all_orientations, *width, *height, counts))
        .count()
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
        let input = read_example(12);
        assert_eq!(part_one(&input), 2);
    }
}
