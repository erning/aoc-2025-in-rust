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

use std::collections::HashSet;

// Check if a point is inside or on the boundary of the polygon
fn point_in_polygon(red_tiles: &[(i64, i64)], x: i64, y: i64) -> bool {
    let n = red_tiles.len();

    // Check if on boundary
    for i in 0..n {
        let (x1, y1) = red_tiles[i];
        let (x2, y2) = red_tiles[(i + 1) % n];

        if x1 == x2 {
            // Vertical segment
            let (miny, maxy) = (y1.min(y2), y1.max(y2));
            if x == x1 && y >= miny && y <= maxy {
                return true;
            }
        } else {
            // Horizontal segment
            let (minx, maxx) = (x1.min(x2), x1.max(x2));
            if y == y1 && x >= minx && x <= maxx {
                return true;
            }
        }
    }

    // Ray casting for interior - count crossings to the left
    // Use the standard rule: count vertical segment if one endpoint is strictly above
    // and the other is at or below the ray
    let mut crossings = 0;
    for i in 0..n {
        let (x1, y1) = red_tiles[i];
        let (x2, y2) = red_tiles[(i + 1) % n];

        if x1 == x2 && x1 < x {
            // Vertical segment to the left of point
            // Count if one endpoint strictly above, one at or below
            if (y1 < y && y2 >= y) || (y2 < y && y1 >= y) {
                crossings += 1;
            }
        }
    }

    crossings % 2 == 1
}

fn is_rect_fully_inside(
    red_tiles: &[(i64, i64)],
    lx: i64,
    rx: i64,
    ly: i64,
    ry: i64,
) -> bool {
    // Quick check: all four corners must be inside
    if !point_in_polygon(red_tiles, lx, ly) {
        return false;
    }
    if !point_in_polygon(red_tiles, rx, ly) {
        return false;
    }
    if !point_in_polygon(red_tiles, lx, ry) {
        return false;
    }
    if !point_in_polygon(red_tiles, rx, ry) {
        return false;
    }

    // Check all points on edges at key y values
    let n = red_tiles.len();
    let mut key_y: HashSet<i64> = HashSet::new();
    for &(_, y) in red_tiles {
        if y > ly && y < ry {
            key_y.insert(y);
        }
    }

    for y in key_y {
        // Check left and right edges at this y
        if !point_in_polygon(red_tiles, lx, y) {
            return false;
        }
        if !point_in_polygon(red_tiles, rx, y) {
            return false;
        }
    }

    // Check that no polygon edge crosses through the rectangle interior
    for i in 0..n {
        let (x1, y1) = red_tiles[i];
        let (x2, y2) = red_tiles[(i + 1) % n];

        if x1 == x2 {
            // Vertical segment
            let (miny, maxy) = (y1.min(y2), y1.max(y2));
            // Check if this segment is strictly inside the rectangle horizontally
            if x1 > lx && x1 < rx {
                // Check if it crosses the rectangle vertically
                if miny < ry && maxy > ly {
                    // This segment is inside the rectangle - it divides it
                    // The rectangle is only valid if the segment is on the boundary
                    // i.e., it goes all the way through
                    if !(maxy >= ry && miny <= ly) {
                        return false;
                    }
                }
            }
        } else {
            // Horizontal segment
            let (minx, maxx) = (x1.min(x2), x1.max(x2));
            // Check if this segment is strictly inside the rectangle vertically
            if y1 > ly && y1 < ry {
                // Check if it crosses the rectangle horizontally
                if minx < rx && maxx > lx {
                    // This segment is inside the rectangle
                    if !(maxx >= rx && minx <= lx) {
                        return false;
                    }
                }
            }
        }
    }

    true
}

pub fn part_two(input: &str) -> i64 {
    let red_tiles = parse_input(input);
    let n = red_tiles.len();

    let mut max_area = 0;

    // For each pair of red tiles
    for i in 0..n {
        for j in (i + 1)..n {
            let (x1, y1) = red_tiles[i];
            let (x2, y2) = red_tiles[j];

            let lx = x1.min(x2);
            let rx = x1.max(x2);
            let ly = y1.min(y2);
            let ry = y1.max(y2);

            if is_rect_fully_inside(&red_tiles, lx, rx, ly, ry) {
                let area = (rx - lx + 1) * (ry - ly + 1);
                max_area = max_area.max(area);
            }
        }
    }

    max_area
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(9);
        assert_eq!(part_one(&input), 50);
        assert_eq!(part_two(&input), 24);
    }
}
