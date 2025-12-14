#!/bin/bash
# Download Advent of Code input for a specific day
# Usage: ./scripts/download-input.sh <day>
#
# Requires AOC_SESSION environment variable or .aoc-session file
# To get your session cookie:
# 1. Login to https://adventofcode.com via GitHub
# 2. Open browser DevTools (F12) -> Application -> Cookies
# 3. Copy the value of 'session' cookie

set -e

YEAR=2025
DAY=${1:-$(date +%-d)}

# Pad day to 2 digits
DAY_PADDED=$(printf "%02d" "$DAY")

# Get session token
if [ -n "$AOC_SESSION" ]; then
    SESSION="$AOC_SESSION"
elif [ -f ".aoc-session" ]; then
    SESSION=$(cat .aoc-session)
elif [ -f "$HOME/.aoc-session" ]; then
    SESSION=$(cat "$HOME/.aoc-session")
else
    echo "Error: No session token found."
    echo "Set AOC_SESSION env var or create .aoc-session file"
    exit 1
fi

OUTPUT="inputs/${DAY_PADDED}-input.txt"
URL="https://adventofcode.com/${YEAR}/day/${DAY}/input"

echo "Downloading Day ${DAY} input..."
curl -s -b "session=${SESSION}" "$URL" -o "$OUTPUT"

if grep -q "Puzzle inputs differ by user" "$OUTPUT" 2>/dev/null; then
    echo "Error: Invalid session token"
    rm "$OUTPUT"
    exit 1
fi

if grep -q "Please don't repeatedly request this endpoint" "$OUTPUT" 2>/dev/null; then
    echo "Error: Rate limited or puzzle not yet available"
    rm "$OUTPUT"
    exit 1
fi

echo "Saved to ${OUTPUT}"
