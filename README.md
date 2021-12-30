# Advent of Code 2021, in Rust

This is the repo for my solutions to [2021's advent of code](https://adventofcode.com/2021). 

Have a look around, read the [usage section](#usage) if you're planning to run any of the stages.

I've written some notes about some of the specific solutions in the [Solution Notes](#solution-notes) section.

# Usage

To run the examples, you'll need:

* The rust toolchain installed, **1.58.0 or greater**(*)
* The `libssl` and `pkg-config` packages installed (for linux platforms)
* Some source of samples (see the sections below)

  (*) - At time of writing, the current stable toolchain is 1.57.0. This is missing some required language features, and will cause some solutions to fail to build. Toolchains 1.58.0 or greater will work - beta or nightly. You can make sure you're using the beta toolchain by running `rustup toolchain add beta && rustup default beta`.

Any day's solution can then be run with a command of the form -

```
cargo run --release --bin day1
```

This command runs day 1's solution. Replace day1 with `day2`, `day3` to get different solutions.

## Getting inputs with an advent of code account

You'll need to take your advent of code session cookie, and create a `.env` file containing this cookie.

You can find your session cookie by logging into the AoC website, and opening your browser's dev tools using f12. In chrome, you can find your session cookie in the `Application` tab, under `Storage -> Cookies` on the left. 

Copy the **value** field from your cookie, and make a file `.env` that looks like this:

```
AOC_SESSION=12345abcdef123410asdfasdasdasfasdasfasdf
```

## Inputs without an advent of code account

My puzzle inputs are included with this repository, for people who want to run the solutions without needing to actually make env files or need an AoC account.

You can copy the `inputs` directory to a new directory, named `.cache`, which will then be automatically used by the solutions, when run.

On linux: 
```
cp -r inputs .cache
```

# Solution Notes and Acknowledgements

Some solutions are cleaner than others, depending on how much time I could dedicate to the problems. 

I managed to solve most or all of the problems within 24 hours, and none more than 48 hours.

Here are some thoughts on some specific solutions, or acknowledgements where appropriate - 

## day 6

[code](src/bin/day6.rs)

[problem](https://adventofcode.com/2021/day/6)

The code looks clunky, but it's quite fast - using an array for the shoal of lantern fish would not hugely reduce the verbosity of the code, or it would make it run slower, but, why use a fast language if you don't want to write fast code?

## day 8

[code](src/bin/day8.rs)

[problem](https://adventofcode.com/2021/day/8)

This one was quite fun, it was the first of the "hard" problems for this year's AoC. The process of writing inductive reasoning as code (*"this combination of segments only has two overlapping segments with this known character, therefore it must represent this character"*) is quite satisfying - there's a fairly wordy section in the code which uses a series of set operations to gradually deduce which characters are which.

## day 9

[code](src/bin/day9.rs)

[problem](https://adventofcode.com/2021/day/9)

Recursion wasn't really necessary here, but it was fun to apply it, and stack depth never really ended up being an issue. It's also quite fast!

I created a visualisation for this problem - you can run `cargo run --example day9-gif --features image` and it will (after a few minutes) spit out a fairly large gif of the search algorithm walking through points on the map.

## day 10

[code](src/bin/day10.rs)

[problem](https://adventofcode.com/2021/day/10)

Much like recursion, it's quite satisfying when you get to implement a stack.

## day 12

[code](src/bin/day12.rs)

[problem](https://adventofcode.com/2021/day/12)

More recursion! This would have been absolutely trivial with a graph library I'm sure, but I couldn't quite wrap my head around how I'd implement the double-visit mechanics of the second question in the context of an existing graph library, and the visiting algorithm wasn't too difficult to implement by itself.

The modifications for making the code work for part 2 were in fact remarkably simple, which was nice - you can see that in how the answer for part 1 is solved with the same code as part 1, but with a boolean flag enabled.

## day 15

[code](src/bin/day15.rs)

[problem](https://adventofcode.com/2021/day/15)

A good programmer knows when not to reinvent the wheel, and the world is certainly not short of implementations of dijkstra's algorithm, so this one's a (fairly boring) "make a graph and pass it to the graph library's dijkstra function" implementation.

## day 16

[code](src/bin/day16.rs)

[problem](https://adventofcode.com/2021/day/16)

A lot of packed structs, which is bread-and-butter at my day job. One of the rare cases where it would have been faster to implement in C, purely for the fact that every modern language has removed packed structs and bitfields, to stop programmers from shooting themselves in the feet. It's annoying to not have bitfields in rust, but it's probably for the best.

Someone should tell the elves about protocol buffers.

## day 19

[code](src/bin/day19.rs)

[problem](https://adventofcode.com/2021/day/19)

I put some electrical tape on a 6-sided die and marked `+x`, `-x`, `+y` etc on the faces to figure out the various rotations of a cube. Reading after the fact, there are ways (relating to the parity and permutations of the faces) to generate all the 24 rotations. 

Overall this was a fun problem, but it was one of the harder problems of the series.

## day 21

[code](src/bin/day21.rs)

[problem](https://adventofcode.com/2021/day/21)

This runs incredibly slowly without the `cache_a` and `cache_b` memoization, and virtually instantly with. I probably could have defined a type for the cached states (instead of `(usize, usize, usize, usize, usize)`), but I'm leaving it as-is.

## day 22

[code](src/bin/day22.rs)

[problem](https://adventofcode.com/2021/day/22)

This was an odd one, and the problem that took me the longest (overall) to solve. This boiled down to a handful of false-starts attempting to represent the space of cubes as smaller, broken-down cubes, that were created when larger cubes were "split" by new cubes overlapping them. 

Took me some time to figure out that I already had a list of all the sub-cubes - by iterating all the combinations of x, y and z coordinates, it would give me all the sub-cubes, and I simply had to add up the cubes that corresponded to an "on" state.

## day 23

[code for p1](src/bin/day23-p1.rs) - [code for p2](src/bin/day23-p2.rs)

[problem](https://adventofcode.com/2021/day/23)

This was a tricky problem, and credit goes to my good friend stephen for his suggestion of using a directed graph to represent the space of all possible "amphipod burrow" states. The complexity of the code means that I ended up dividing parts 1 and 2 into separate files.

The vast majority of the computation is spent in constructing the graph - testing valid moves of amphipods, and adding nodes and edges where appropriate. The actual pathfinding is, as always, quite fast.

In retrospect I could add some heuristics to speed up the graph construction process - summing the distance of the current "branch" and exiting early if the score doesn't beat the current best. 

The whole thing could probably also benefit from some memoization, although rust isn't particularly well-suited to it. I'd pay good money for a `@lru_cache` decorator :thinking:

## day 24

[code](src/bin/day24.rs)

[problem](https://adventofcode.com/2021/day/24)

On release, this problem received some criticism as it was, effectively, not a problem that could be reasonably solved in every language - it has **very** large search space (`22,876,792,454,961`) which is on the edge of what current computers are comfortable with brute-forcing. 

This meant that it wasn't (naively) solvable in inefficient languages like python, and most people ended up solving their problems by hand (by figuring out the input data and intuiting the answer) or just using an SMT solver like Z3.

I've *just about* managed to write a general solution to this problem - it generates code directly from the problem input, and exploits the fact that LLVM's compile- and link-time-optimisation is, without hyperbole, **incredible**. This means that this solution can generalise to any problem input - which is, as far as I can gather, a fairly rare approach to this problem. My implementation brings the runtime down to a few tens of minutes on an old laptop, which isn't too bad.

Worth remembering that I was solving this problem on christmas eve, so there's certainly some readability changes to be made.

As is always the case on the internet, someone else has done it better - [this is a blog post](https://www.mattkeeter.com/blog/2021-12-27-brute/) where someone brought their runtime down to the order of seconds, which is significantly better than mine. Interestingly, it also uses rust.

# Conclusion

I had lots of fun writing this and participating in AoC, and I look forward to next year's!

# Troubleshooting

* **Day 13 fails to build - it complains about trait bounds for `line.split`!**
    * Make sure you're using rust version 1.58.0 or greater. This might mean installing the beta or nightly toolchain.