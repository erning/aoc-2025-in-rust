const DIAL_SIZE: i32 = 100; // Total number of positions on the dial (0-99)

fn parse_input(input: &str) -> Vec<(char, i32)> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let direction = line.chars().next().unwrap();
            let number = line[1..].parse::<i32>().unwrap();
            (direction, number)
        })
        .collect()
}

pub fn part_one(input: &str) -> i32 {
    let instructions = parse_input(input);
    let mut position = 50; // initial position
    let mut zero_count = 0; // count of stops at 0

    for (direction, steps) in instructions {
        match direction {
            'L' => {
                // turn left: counter-clockwise, position decreases
                position = (position - steps) % DIAL_SIZE;
            }
            'R' => {
                // turn right: clockwise, position increases
                position = (position + steps) % DIAL_SIZE;
            }
            _ => panic!("Invalid direction: {}", direction),
        }

        // check if stopped at 0
        if position == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

pub fn part_two(input: &str) -> i32 {
    let instructions = parse_input(input);
    let mut position = 50; // initial position
    let mut zero_count = 0; // count of all passes through 0

    for (direction, steps) in instructions {
        // Calculate complete rotations and remainder
        let full_rotations = steps / 100;
        let remainder = steps % 100;

        zero_count += full_rotations; // Each full rotation passes through 0

        if remainder > 0 {
            let new_position = match direction {
                'L' => {
                    let new_pos =
                        (position + DIAL_SIZE - remainder) % DIAL_SIZE;
                    // Count crossing 0 if not starting from 0 and not landing on 0
                    if position != 0 && position < remainder && new_pos != 0 {
                        zero_count += 1;
                    }
                    new_pos
                }
                'R' => {
                    let new_pos = (position + remainder) % DIAL_SIZE;
                    // Count crossing 0 if not starting from 0 and not landing on 0
                    if position != 0
                        && position + remainder >= DIAL_SIZE
                        && new_pos != 0
                    {
                        zero_count += 1;
                    }
                    new_pos
                }
                _ => panic!("Invalid direction: {}", direction),
            };

            // Count if we land exactly on 0
            if new_position == 0 {
                zero_count += 1;
            }

            position = new_position;
        }
    }

    zero_count
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
