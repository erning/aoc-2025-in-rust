use std::collections::HashMap;

fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for line in input
        .lines()
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
    {
        let parts: Vec<&str> = line.split(':').collect();
        let from = parts[0].trim().to_string();
        let to: Vec<String> =
            parts[1].split_whitespace().map(|s| s.to_string()).collect();
        graph.insert(from, to);
    }
    graph
}

fn count_paths(
    graph: &HashMap<String, Vec<String>>,
    from: &str,
    to: &str,
) -> u64 {
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

// Count paths that visit both 'dac' and 'fft'
// paths(svr -> out via dac & fft) =
//   paths(svr -> dac) * paths(dac -> fft) * paths(fft -> out) +
//   paths(svr -> fft) * paths(fft -> dac) * paths(dac -> out)
fn count_paths_via_two(
    graph: &HashMap<String, Vec<String>>,
    from: &str,
    to: &str,
    via1: &str,
    via2: &str,
) -> u64 {
    // Path: from -> via1 -> via2 -> to
    let p1 = count_paths(graph, from, via1)
        * count_paths(graph, via1, via2)
        * count_paths(graph, via2, to);
    // Path: from -> via2 -> via1 -> to
    let p2 = count_paths(graph, from, via2)
        * count_paths(graph, via2, via1)
        * count_paths(graph, via1, to);
    p1 + p2
}

pub fn part_one(input: &str) -> u64 {
    let graph = parse_input(input);
    count_paths(&graph, "you", "out")
}

pub fn part_two(input: &str) -> u64 {
    let graph = parse_input(input);
    count_paths_via_two(&graph, "svr", "out", "dac", "fft")
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

    #[test]
    fn example2() {
        let input = crate::read_as_string(11, "example2");
        assert_eq!(part_two(&input), 2);
    }
}
