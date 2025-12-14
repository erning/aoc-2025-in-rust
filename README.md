# Advent of Code 2025 in Rust

My solutions to [Advent of Code 2025](https://adventofcode.com/2025/) puzzles, implemented in Rust.

## Running Solutions

```bash
# Run all days
cargo run --release --

# Run specific days
cargo run --release -- 1 2 3

# Run with example inputs
cargo run --release -- --example
```

## Testing

```bash
# Run all tests
cargo test

# Run tests for a specific day
cargo test day01
```

## Progress

| Day | Part One | Part Two | Title |
|:---:|:--------:|:--------:|-------|
| [1](https://adventofcode.com/2025/day/1) | :star: | :star: | Historian Hysteria |
| [2](https://adventofcode.com/2025/day/2) | :star: | :star: | Gift Shop |
| [3](https://adventofcode.com/2025/day/3) | :star: | :star: | Lobby |
| [4](https://adventofcode.com/2025/day/4) | :star: | :star: | Printing Department |
| [5](https://adventofcode.com/2025/day/5) | :star: | :star: | Cafeteria |
| [6](https://adventofcode.com/2025/day/6) | :star: | :star: | Trash Compactor |
| [7](https://adventofcode.com/2025/day/7) | :star: | :star: | Laboratories |
| [8](https://adventofcode.com/2025/day/8) | :star: | :star: | Playground |
| [9](https://adventofcode.com/2025/day/9) | :star: | :star: | Movie Theater |
| [10](https://adventofcode.com/2025/day/10) | :star: | :star: | Factory |
| [11](https://adventofcode.com/2025/day/11) | :star: | :star: | Reactor |
| [12](https://adventofcode.com/2025/day/12) | :star: | :star: | Christmas Tree Farm |

## Algorithm Analysis

| Day | Algorithm | Complexity | Notes |
|:---:|-----------|------------|-------|
| 1 | Linear traversal | O(n) | Modular arithmetic for dial position |
| 2 | Generative enumeration | O(range) | Generate doubled/repeated numbers directly |
| 3 | Greedy selection | O(n×k) | Pick largest available digit at each position |
| 4 | Grid simulation | O(n×m×iter) | Iteratively remove accessible cells |
| 5 | Interval merging | O(n log n) | Sort and merge overlapping ranges |
| 6 | Parsing | O(input) | Column-wise number extraction |
| 7 | HashMap beam tracking | O(n×m) | Track timeline counts per position |
| 8 | Union-Find + sorting | O(n² log n) | Kruskal-style edge processing |
| 9 | Point-in-polygon | O(n²×k) | Ray casting for rectangle validation |
| 10 | Gaussian elimination + search | O(n³ + search) | Integer linear system with pruned search |
| 11 | Memoized DFS | O(V+E) | Path counting with caching |
| 12 | Backtracking | Exponential | Polyomino fitting with first-empty-cell heuristic |

