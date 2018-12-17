### Advent of Code 2018 (in Rust)

My solutions for the 2018 edition of the [Advent of code](https://adventofcode.com/2018), all implemented in [Rust](https://www.rust-lang.org/).

As I try to implement each problem within twenty four hours after its publication, most of this code is quick and dirty.

# Noticeable solutions:

I implemented a custom 1D version of the [HashLife algorithm](https://en.wikipedia.org/wiki/Hashlife) for the second part of day12. On my particular input it can compute 1000000000000000000000000000000000000 iterations in a fraction of a second before hitting overflow problems (with 128bit integers).
