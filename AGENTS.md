# CLAUDE.md

## Project Overview
This is a template project for Advent of Code implementation in Rust, featuring solutions with consistent architecture and testing patterns.

## Architecture
- **Entry Point**: `src/main.rs` - Main executable with option to run all days or specific ones
- **Library**: `src/lib.rs` - Re-exports day modules and provides I/O utilities  
- **Day Modules**: `src/dayXX.rs` - Individual solutions following standardized patterns
- **Inputs**: `inputs/` - Example and actual puzzle inputs organized by day

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
