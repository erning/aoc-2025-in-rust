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
                position = (position - steps) % 100;
            }
            'R' => {
                // turn right: clockwise, position increases
                position = (position + steps ) % 100;
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
        match direction {
            'L' => {
                // turn left: counter-clockwise, position decreases
                for _ in 0..steps {
                    position = (position - 1) % 100;
                    if position == 0 {
                        zero_count += 1;
                    }
                }
            }
            'R' => {
                // turn right: clockwise, position increases
                for _ in 0..steps {
                    position = (position + 1) % 100;
                    if position == 0 {
                        zero_count += 1;
                    }
                }
            }
            _ => panic!("Invalid direction: {}", direction),
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
