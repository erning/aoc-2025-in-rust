const DIAL_SIZE: i32 = 100;

fn parse_input(input: &str) -> Vec<(i32, i32)> {
    // Parse direction as sign: R=+1, L=-1
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let sign = if line.starts_with('R') { 1 } else { -1 };
            let steps = line[1..].parse::<i32>().unwrap();
            (sign, steps)
        })
        .collect()
}

pub fn part_one(input: &str) -> i32 {
    let instructions = parse_input(input);
    let mut position = 50;
    let mut count = 0;

    for (sign, steps) in instructions {
        position = (position + sign * steps).rem_euclid(DIAL_SIZE);
        if position == 0 {
            count += 1;
        }
    }

    count
}

pub fn part_two(input: &str) -> i32 {
    let instructions = parse_input(input);
    let mut position = 50;
    let mut count = 0;

    for (sign, steps) in instructions {
        // Each full rotation crosses 0 once
        count += steps / DIAL_SIZE;

        let remainder = steps % DIAL_SIZE;
        let new_position =
            (position + sign * remainder).rem_euclid(DIAL_SIZE);

        // Check if we crossed 0 (not counting starting from 0)
        let crossed = if position != 0 {
            if sign > 0 {
                position + remainder >= DIAL_SIZE
            } else {
                position < remainder
            }
        } else {
            false
        };

        // Landing on 0 counts (but not if we started from 0 and didn't move)
        let landed_on_zero =
            new_position == 0 && (position != 0 || remainder > 0);

        if crossed || landed_on_zero {
            count += 1;
        }

        position = new_position;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(1);
        assert_eq!(part_one(&input), 3);
        assert_eq!(part_two(&input), 6);
    }
}
