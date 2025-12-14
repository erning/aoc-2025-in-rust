use std::collections::HashMap;

fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines().filter(|l| !l.is_empty()) {
        let parts: Vec<&str> = line.split(':').collect();
        let from = parts[0].trim().to_string();
        let to: Vec<String> = parts[1].split_whitespace().map(|s| s.to_string()).collect();
        graph.insert(from, to);
    }
    graph
}

fn count_paths(graph: &HashMap<String, Vec<String>>, from: &str, to: &str) -> u64 {
    // Use memoization with DFS
    let mut memo: HashMap<String, u64> = HashMap::new();
    count_paths_dfs(graph, from, to, &mut memo)
}

fn count_paths_dfs(
    graph: &HashMap<String, Vec<String>>,
    current: &str,
    target: &str,
    memo: &mut HashMap<String, u64>,
) -> u64 {
    if current == target {
        return 1;
    }

    if let Some(&count) = memo.get(current) {
        return count;
    }

    let mut total = 0;
    if let Some(neighbors) = graph.get(current) {
        for neighbor in neighbors {
            total += count_paths_dfs(graph, neighbor, target, memo);
        }
    }

    memo.insert(current.to_string(), total);
    total
}

pub fn part_one(input: &str) -> u64 {
    let graph = parse_input(input);
    count_paths(&graph, "you", "out")
}

pub fn part_two(_input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(11);
        assert_eq!(part_one(&input), 5);
    }
}
