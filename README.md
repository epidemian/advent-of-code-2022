# Advent of Code 2022

An attempt at [2022's Advent of Code](https://adventofcode.com/2022) challenge and an excuse to learn and practice Rust.

I recorded myself solving these puzzles to try to maintain focus and finish them in a reasonable time. Sort of like a Pomodoro-on-steroids. You may see the cringy results [here](https://www.youtube.com/playlist?list=PL3kymB6hDjyU2ptzNkLrOxsiBpl-OgDyR).

## Goals

- Have fun
- Learn stuff
- Fast execution time (< 1 second for whole set of puzzles)
- Read inputs from files (or stdin); don't include their contents into the executable
- (optional) Better error handling

## Notes & Learnings

### Day 1: Calorie Counting

Simple starter challenge. I learned how to read an entire file into a string :)

### Day 2: Rock Paper Scissors

I overcomplicated the solution at first but then managed to find a better way to express part 2's logic re-using the function to compute the round score used for part 2.

I learned/remembered that enum values can be cast to their corresponding integer values by using the `as` operator. E.g., `anEnumValue as u32`.

### Day 3: Rucksack Reorganization

Learned about slice::split_at(n), which is very convenient for splitting things without allocating. And also that there's no good solution for chunking an iterator on Rust stable channel yet.

### Day 4: Camp Cleanup

This one was a simple boolean logic puzzle. A nice breather before harder things to come for sure.

Learned that using str::split() and collecting the results into a dynamic Vec and then pattern-matching a full slice of that array into variables is a very convenient way of parsing some of these puzzles :)
