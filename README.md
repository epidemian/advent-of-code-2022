# Advent of Code 2022

An attempt at [2022's Advent of Code](https://adventofcode.com/2022) challenge and an excuse to learn and practice Rust.

I recorded myself solving these puzzles to try to maintain focus and finish them in a reasonable time. Sort of like a Pomodoro-on-steroids. You may see the cringy results [here](https://www.youtube.com/playlist?list=PL3kymB6hDjyU2ptzNkLrOxsiBpl-OgDyR).

## Goals

- Have fun
- Learn stuff
- Fast execution time (< 1 second for whole set of puzzles)
- Read inputs from files (or stdin); don't include their contents into the executable
- (optional) Better error handling
- (optional) Use vanilla Rust; no external dependencies

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

Update: learned about `Vec::split_off(ind)` and `Vec::extend()`, which are a bit more clear and expressive than `Vec::drain().collect()` and `Vec::append()` respectively, although in terms of performance they should be basically the same.

### Day 6: Tuning Trouble

A much simpler puzzle that the previous one. Almost no parsing required, and the uniqueness check was straightforward to implement using a HashSet. There are surely optimization tricks that could be applied if we wanted to make this algorithm more optimal, like keeping track of the last N characters in a `HashMap<u8, usize>` and not having to recompute (and re-hash) the unique characters on each step.

### Day 7: No Space Left On Device

This was a tricky one. At first i tried building a tree data structure from the terminal output, but that approach soon became too complicated and then i abandoned [in favor of using a much simpler data structure](src/day7.rs) for the FS nodes, where the directory sizes were calculated directly while parsing the terminal output and stored in a flat `HashMap<String, usize>` where the keys were full directory paths like `"/foo/bar"`.

After getting the answers with the simple and direct solution, i tried [modelling the FS with a tree](src/day7_tree.rs) again, and although that approach required more code and is in general more complicated, i am happy to at least having figured it out :)

The latter approach required some rather tricky bookkeeping of mutable references while building the tree. And i was almost sure i was not going to convince the borrow checker that everything was fine and was going to need some escape-hatch like using `Cell`s or something like that. But luckily no such hacks were needed: all it took was learning about `ref` bindings.

I found it also a bit cumbersome to implement the `FsNode::walk(fn)` function, or rather to declare its type correctly. I guess i could have also tried implementing a custom iterator for `FsNode`, but that seemed even more daunting.

### Day 8: Treetop Tree House

A nice grid-based puzzle. Learned about using `an_usize_val.wrapping_add_signed(an_isize_val)` for doing maths without needing to do lots of type conversions between signed and unsigned integers. Also learned about the `.product()` iterator function.

### Day 9: Rope Bridge

A simple 2D movement puzzle. I really liked the generalization of the rope length needed for Part 2. Got an excuse to learn about and use `std::iter::repeat()` to flatten the step moves into a flat iterator :)

### Day 10: Cathode-Ray Tube

This one was a super fun one. Part 1 was quite easy, while part 2 had an unexpected "twist" of —spoiler alert!— reinterpreting the data of part 1 as an image.

### Day 11: Monkey in the Middle

This one involved tedious amounts of parsing. After that, the simulation for the first part was straightforward, and then the second part involved a neat modular math trick.

I'm quite satisfied with the end result, where some OO-ish delegation was used to spread the parsing logic into smaller chunks instead of a giant mess of `unwrap()`/`expect(..)`-riddled single function. And also, this puzzle turned out to be a great excuse to nerd it out on data structures, with valid uses of `VecDeque`, `HashMap` and even `BinaryHeap`! 🤓

### Day 12: Hill Climbing Algorithm

A deceptive title for a path-finding problem. I initially reused (i.e., copy-pasted) a Dijkstra's algorithm implementation [from my last-year's AoC solution](https://github.com/epidemian/advent-of-code-2021/blob/main/src/dijkstra.rs), which worked alright, but needed to brute-force the solution to part 2 by computing the shortest path from any starting point of height 0 (around 600 points on the given input) to the end point.

This brute-forcing meant a subpar runtime performance and a noticeable delay when running in debug mode (~1.6 seconds). After some reading and time spent on a chin-grabbing thinking pose, i could adapt that algorithm to compute all distances to the end point in one go and use those computed distances for both part 1 and 2. This improved performance significantly, reducing the runtime for this puzzle to around the same ballpark of any of the previous ones :)

### Day 13: Distress Signal

This one was a very nice excuse to implement a custom ordering logic through the Ord/PartialOrd traits. I also learned how to do slice patterns like those typical in functional programming using the `@` operator to capture a part of a pattern: `let [head, tail @ ..] = a_slice else { handle_empty_case() }`.

Initially, i used [`serde_json`](https://docs.rs/serde_json/latest/serde_json/) to quickly parse the inputs as JSON arrays, but then i implemented a custom ad-hoc parser in the form of a very basic loop to avoid the need for an external dependency.

### Day 14: Regolith Reservoir

Nice sand-falling simulation. Went for a straightforward imperative solution. Also enjoyed playing around with [animating the falling sand](https://youtu.be/1-3rK491974) in the terminal. To enable this animation, set the `ANIMATE` environment var: `ANIMATE=1 cargo run 14`.

### Day 15: Beacon Exclusion Zone

Interesting big-numbers crunching problem. For part 1 i could get away with a very brute-force solution of checking every position along the given line. But for part 2 i needed to come up with a more clever approach, which was pretty difficult TBH.

My current solution is still a little brute-forcey —just that instead of checking all positions across a line, i'm checking all lines across the 4M needed range—but at least it runs in a reasonable time (~200ms). Still, i think a much more direct solution is possible if we could model the intersection of the 2D beacon exclusion zones better.
