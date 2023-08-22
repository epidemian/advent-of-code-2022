# Advent of Code 2022

An attempt at [2022's Advent of Code](https://adventofcode.com/2022) challenge and an excuse to learn and practice Rust.

I recorded myself solving these puzzles to try to maintain focus and finish them in a reasonable time. Sort of like a Pomodoro-on-steroids. You may see the cringy results [here](https://www.youtube.com/playlist?list=PL3kymB6hDjyU2ptzNkLrOxsiBpl-OgDyR).

## Goals

- Have fun
- Learn stuff
- Fast execution time (< 1 second for whole set of puzzles)
- Read inputs from files (or stdin); don't include their contents into the executable
- (optional) Better error handling
- (optional) Use vanilla Rust; no external dependencies, except for a couple of exceptions:
  - Rayon, which is awesome :)
  - fxhash, which is just the std `Hash{Set,Map}` but faster.

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

### Day 16: Proboscidea Volcanium

Freaking hardest one so far by far. It took me many attempts, and i only managed to get part 1 answer by myself. In the end i ended up "cheating" and looking for other solutions online. I'm glad i did though, as [the solution i liked the most](https://old.reddit.com/r/adventofcode/comments/zn6k1l/2022_day_16_solutions/j2xhog7/), and ended up copying, was super elegant but something i think i would never have come up with on my own. If i could send a hint to myself retroactively in time i would just say: "you can represent the opened valves with a bitmask 😉" and maybe that would have been enough to get it.

I learned that Rust doesn't allow recursive closures. At least not in a direct and intuitive way.

### DAy 17: Pyroclastic Flow

A nice Tetris-like rock fall simulation. Part 1 was relatively straightforward, but part 2 had a twist that required some cleverness to get the answer by detecting cycles on the rock falling patterns instead of simulating 1 trillion rock falls, which would've taken too much computing time.

For kicks and giggles, i also changed the solution to use a binary representation for the chamber and rock, using a single byte for each row, and bitwise operations to detect collisions and the like. Not really a necessary performance improvement or anything, but a fun experiment nevertheless :)

### Day 18: Boiling Boulders

A relatively simple 3D flood-filling problem. I noticed that when working with 2D or 3D grids, there seems to always be some tradeoff when picking either `usize`s or signed integers to represent the points on the grid. When using `usize` variables, no conversion is needed to access or write to the grid, but moving around it is cumbersome as you need to take care of possible underflows when going below 0. And when using signed integer ariables, moving around is easy, as you can subtract without worry of underflows and just do the bounds-checks before accessing the grid, but on the flip side you need to do lots of conversions to and from `usize` when accessing the grid. There's no free lunch.

### Day 19: Not Enough Minerals

An nice <abbr title="Real-time strategy">RTS</abbr>-themed puzzle based on creating robots that collect resources and optimizing the end amount of one of those resources: geodes. A totally naive implementation of pure brute-forcing all state possibilities was too slow, even having the power of an efficiently-compiled language at my disposal, so this became mostly a puzzle of trying to prune the search tree of unnecessary branches.

The interesting pruning "optimizations" were:

- Discarding a state node completely if there's no chance of it achieving more geodes than what's the current maximum found on another branch. This was done in a very conservative way, considering the best-case scenario —producing geode-cracking robots every minute till the end— but it was still surprisingly efficient in cutting off the runtime from several seconds to ~1 sec.
- In the cases where there are enough materials to build a robot type (e.g. ore) but we wait one minute instead, ignore the case of building that robot type on the next minute, as it doesn't make sense to build the same thing later if we have the possibility of building it on a turn where we just waited.
- Not building robots of a certain type if there's already enough of them to build any robot we want on a single turn. This doesn't apply to geode-cracking robots, obviously, since we always want more of those. And it doesn't apply to obsidian robots either, but only because in the 32 simulated minutes, we don't reach any state where this condition of producing enough obsidian to build a geode robot per minute is met. It is applied to ore and clay robots though, and it made a huge difference in the final runtime, lowering it from ~0.5s to ~15ms :)

Besides that, this was also a fun challenge of algorithmic modelling. I started doing a depth-first search in a recursive fashion. But after reaching an efficient implementation that way, i rewrote that main search function to use a classic DFS imperative loop and a reified stack variable, and i liked the result much better. The recursive version needed quite a bit of parameters to pass between one state and the next, while the imperative version can just use local variables for that, which i think is easier to keep track of.

### Day 20: Grove Positioning System

This one had a pretty easy part 1 followed by a part 2 that made the simple approach on part 1 totally unfeasible. Luckily, with a modular arithmetic trick, part 2 became achievable, and after some tweaking and optimization using a sort of doubly-linked list i managed to a fast enough runtime.

### Day 21: Monkey Math

A challenging part 2. I used a recursive function that "solves" a given "variable" (monkey name) up the chain of uses until it reaches the root monkey.

### Day 22: Monkey Map

Maddeningly hard part 2. I ended doing an ad-hoc solution for the particular cube unfolding shape of my input.

Learned something new about Rust: you can declare an immutable variable and initialize later. This can be useful, for example, when you want to assign the variable in different branches of an `if` or `match`. Rush checks that the variable is always assigned, and only once :)

### Day 23: Unstable Diffusion

Kind of a breather after the previous one. Part 1 was a relatively simple cellular automaton-ish simulation. And part 2 was trivial to do after having part 1. Initially i ended up with a "slow" runtime of ~1s, but after changing the main HashSet for elves' positions to a direct-access 2D grid, the runtime dropped to an acceptable ~150ms.

### Day 24: Blizzard Basin

Pretty fun challenge. I'm glad for having realized that you don't really need to simulate all blizzards moving around the map, and instead can determine if a tile has a blizzard at a given time from the starting map alone by doing some simple modular arithmetic.

I'm impressed by how expressive Rust closures can be, without having to worry about memory management or unnecessary allocations. The closure used for the next moves' calculation in the Dijkstra's algorithm looks quite dynamic, but it's actually all stack values that get copied around; no dynamic heap allocations :)

### Day 25: Full of Hot Air

Relatively simple puzzle for last day. The conversion from integers to the weird SNAFU numbers was a bit tricky, but could finally get it working after some trial and error.
