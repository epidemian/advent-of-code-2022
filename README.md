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

I learned/remembered that enum values can be cast to their corresponding integer values by using the `as` operator. E.g., `an_enum_value as u32`.

### Day 3: Rucksack Reorganization

Learned about `slice::split_at(n)`, which is very convenient for splitting things without allocating. And also that there's no good solution for chunking an iterator on Rust stable channel yet.

### Day 4: Camp Cleanup

This one was a simple boolean logic puzzle. A nice breather before harder things to come for sure.

Learned that using `str::split()` and collecting the results into a dynamic Vec and then pattern-matching a full slice of that array into variables is a very convenient way of parsing some of these puzzles :)


### Day 5: Supply Stacks

Some pretty involved parsing logic, but then the simulation of crane moves was straightforward enough.

I did struggle trying to find an optimal way of doing the bulk moves of part 2, but in the end preferred to use a simple `Vec::drain().collect()` + `Vec::append()` combo to have the logic expressed in a straightforward way, although incurring in an unnecessary allocation for the temporary vector of moved crates. A "better" solution for that would have required to [split the mutable borrow](https://doc.rust-lang.org/nomicon/borrow-splitting.html) of the `stacks` vector, to have two different mutable borrows: one for the "source" crate stack, and another for the "destination" stack, such that elements from the source could be copied into the destination without worry for aliasing. but that seemed like too much hassle TBH.

### Day 6: Tuning Trouble

A much simpler puzzle that the previous one. Almost no parsing required, and the uniqueness check was straightforward to implement using a HashSet. There are surely optimization tricks that could be applied if we wanted to make this algorithm more optimal, like keeping track of the last N characters in a `HashMap<u8, usize>` and not having to recompute (and re-hash) the unique characters on each step.

### Day 7: No Space Left On Device

This was a tricky one. At first i tried building a tree data structure from the terminal output, but that approach soon became too complicated and then i abandoned [in favor of using a much simpler data structure](src/day7.rs) for the FS nodes, where the directory sizes were calculated directly while parsing the terminal output and stored in a flat `HashMap<String, usize>` where the keys were full directory paths like `"/foo/bar"`.

After getting the answers with the simple and direct solution, i tried [modelling the FS with a tree](src/day7_tree.rs) again, and although that approach required more code and is in general more complicated, i am happy to at least having figured it out :)

The latter approach required some rather tricky bookkeeping of mutable references while building the tree. And i was almost sure i was not going to convince the borrow checker that everything was fine and was going to need some escape-hatch like using `Cell`s or something like that. But luckily no such hacks were needed: all it took was learning about `ref` bindings.

I found it also a bit cumbersome to implement the `FsNode::walk(fn)` function, or rather to declare its type correctly. I guess i could have also tried implementing a custom iterator for `FsNode`, but that seemed even more daunting.

### Day 8: Treetop Tree House

A nice grid-based puzzle. Learned about using `an_usize_val.wrapping_add_signed(an_isize_val)` for doing maths without needing to do lots of type conversions between signed and unsigned integers. Also learned about the `.product()` iterator function.
