# AGENTS.md

## Project Overview
This is an [Advent of Code 2025](https://adventofcode.com/2025/) solution project in Rust. AoC 2025 contains 12 days of puzzles (Day 1-12).

- Puzzle page: `https://adventofcode.com/2025/day/${N}`
- Input download: `https://adventofcode.com/2025/day/${N}/input` (requires authentication)

## Architecture
- **Entry Point**: `src/main.rs` - Main executable with option to run all days or specific ones
- **Library**: `src/lib.rs` - Re-exports day modules and provides I/O utilities  
- **Day Modules**: `src/day{NN}.rs` - Individual solutions following standardized patterns
- **Inputs**: `inputs/` - Puzzle inputs organized by day
  - `{NN}-example.txt` - Example input from puzzle description
  - `{NN}-input.txt` - Actual puzzle input (requires authentication to download)

## Core Patterns
Each day module implements:
- Private `parse_input()` function for input parsing
- Public `part_one()` and `part_two()` functions returning results
- Tests using `read_example()` against provided examples

## Commands

### Build & Run
```bash
cargo build --release
cargo run --release --           # Run all days
cargo run --release -- 1 5 10    # Run specific days  
cargo run --release -- --example # Use example inputs
```

### Testing
```bash
cargo test                           # All tests
cargo test day05 -- --nocapture      # Specific day tests
```

### Development
```bash
cargo check    # Quick syntax/type validation
cargo clippy   # Linting suggestions
cargo fmt      # Code formatting
```

### Input Utilities
- `aoc::read_input(day)` - Real puzzle input
- `aoc::read_example(day)` - Example input
- `aoc::read_as_string(day, filename)` - Custom input file

## Scripts

### Download Puzzle Input
```bash
# Download input for a specific day (requires authentication)
./scripts/download-input.sh 5

# Download today's input
./scripts/download-input.sh

# Set up authentication (choose one method):
# Method 1: Environment variable
export AOC_SESSION="your_session_cookie_here"

# Method 2: Local file
echo "your_session_cookie_here" > .aoc-session

# Method 3: Global file
echo "your_session_cookie_here" > ~/.aoc-session
```

To get your session cookie:
1. Login to https://adventofcode.com via GitHub
2. Open browser DevTools (F12) → Application → Cookies
3. Copy the value of 'session' cookie

### Submit Answer
```bash
# Submit answer for a specific day and part
./scripts/submit-answer.sh 5 1 12345

# The script will indicate if the answer is correct or not
# and provide any available hints (too high/low, rate limits, etc.)
```

## Extension Workflow
1. Create `src/dayXX.rs` following established patterns
2. Add module declaration to `src/lib.rs`
3. Register in `src/main.rs` puzzles array
4. Add corresponding input and test files

## Quality Standards
- All code passes `cargo clippy` checks
- Follows `.rustfmt.toml` formatting rules  
- Includes algorithm explanation comments
- Tests validate against provided examples
