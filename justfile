#!/usr/bin/env just --justfile

default: help

help:
    @echo "Usage: just [command]. To create a new day, run 'just new DAY=XX'"


# Command to create a new day. This will copy the template over to a new
# file, update a few funciton  names accordingly, then add this module to
# the puzzles crate.

# Create a new day from the template
new DAY:
    cp src/puzzles/template.rs "src/puzzles/day{{DAY}}.rs"
    echo "pub mod day{{DAY}};" >> src/puzzles.rs
    sed -i ''  -e 's/day_0/day_{{DAY}}/g' "src/puzzles/day{{DAY}}.rs"
    sed -i '' -e 's/day0/day{{DAY}}/g' "src/puzzles/day{{DAY}}.rs"
    sed -i '' -e 's/get_puzzle_input(0/get_puzzle_input({{DAY}}/g' "src/puzzles/day{{DAY}}.rs"
    @echo "Day {{DAY}} created"


# test a specific day
test DAY:
    K9_UPDATE_SNAPSHOTS=1 cargo test "day{{DAY}}"

# test part 1 example
p1e DAY:
    K9_UPDATE_SNAPSHOTS=1 cargo test "day{{DAY}}_p1_example"

# test part 1 real
p1 DAY:
    K9_UPDATE_SNAPSHOTS=1 cargo test "day{{DAY}}_p1_real"

# test part 2 example
p2e DAY:
    K9_UPDATE_SNAPSHOTS=1 cargo test "day{{DAY}}_p2_example"

# test part 2 real
p2 DAY:
    K9_UPDATE_SNAPSHOTS=1 cargo test "day{{DAY}}_p2_real"

# run and watch all tests
watch DAY:
    K9_UPDATE_SNAPSHOTS=1 cargo watch -c -x "test day{{DAY}}"