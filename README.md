# Advent of Code 2025 in Rust

Solutions to [Advent of Code 2025](https://adventofcode.com/2025/) puzzles, implemented in Rust.

All puzzles were solved by [Droid](https://www.factory.ai/) using Claude Opus 4.5 (`claude-opus-4-5-20251101`).

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

## Results

```
--- Day 1: Secret Entrance ---
Part One: 1123
Part Two: 6695

--- Day 2: Gift Shop ---
Part One: 19386344315
Part Two: 34421651192

--- Day 3: Lobby ---
Part One: 16927
Part Two: 167384358365132

--- Day 4: Printing Department ---
Part One: 1527
Part Two: 8690

--- Day 5: Cafeteria ---
Part One: 558
Part Two: 344813017450467

--- Day 6: Trash Compactor ---
Part One: 5977759036837
Part Two: 9630000828442

--- Day 7: Laboratories ---
Part One: 1516
Part Two: 1393669447690

--- Day 8: Playground ---
Part One: 62186
Part Two: 8420405530

--- Day 9: Movie Theater ---
Part One: 4729332959
Part Two: 1474477524

--- Day 10: Factory ---
Part One: 524
Part Two: 21696

--- Day 11: Reactor ---
Part One: 607
Part Two: 506264456238938

--- Day 12: Christmas Tree Farm ---
Part One: 599
Part Two: 0
```

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

