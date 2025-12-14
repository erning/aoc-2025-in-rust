#!/bin/bash
# Submit Advent of Code answer
# Usage: ./scripts/submit-answer.sh <day> <part> <answer>
#
# Requires AOC_SESSION environment variable or .aoc-session file

set -e

YEAR=2025

if [ $# -lt 3 ]; then
    echo "Usage: $0 <day> <part> <answer>"
    echo "Example: $0 2 1 19386344315"
    exit 1
fi

DAY=$1
PART=$2
ANSWER=$3

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

URL="https://adventofcode.com/${YEAR}/day/${DAY}/answer"

echo "Submitting Day ${DAY} Part ${PART}: ${ANSWER}"

RESPONSE=$(curl -s -b "session=${SESSION}" -X POST \
    -d "level=${PART}&answer=${ANSWER}" \
    "$URL")

# Parse response
if echo "$RESPONSE" | grep -q "That's the right answer"; then
    echo "✓ Correct!"
elif echo "$RESPONSE" | grep -q "That's not the right answer"; then
    echo "✗ Wrong answer"
    # Extract hint if available
    echo "$RESPONSE" | grep -oP "your answer is too \w+" || true
elif echo "$RESPONSE" | grep -q "You gave an answer too recently"; then
    echo "⏳ Rate limited - wait before trying again"
    echo "$RESPONSE" | grep -oP "You have \d+[smh]* left to wait" || true
elif echo "$RESPONSE" | grep -q "You don't seem to be solving the right level"; then
    echo "Already completed this level"
else
    echo "Unknown response. Check manually."
fi
