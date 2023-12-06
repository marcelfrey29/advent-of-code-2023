# Advent of Code 2023

[![Continuous Integration](https://github.com/marcelfrey29/advent-of-code-2023/actions/workflows/continuous-integration.yml/badge.svg?branch=main)](https://github.com/marcelfrey29/advent-of-code-2023/actions/workflows/continuous-integration.yml)

https://adventofcode.com/2023

```bash
# Day 01
cargo run --bin day-01 src/bin/day-01-input.txt 0 # Without spelled out numbers
cargo run --bin day-01 src/bin/day-01-input.txt 1 # With spelled out numbers

# Day 02
cargo run --bin day-02 src/bin/day-02-input.txt

# Day 03
cargo run --bin day-03 src/bin/day-03-input.txt

# Day 04
cargo run --bin day-04 src/bin/day-04-input.txt

# Day 05
cargo run --bin day-05 src/bin/day-05-input.txt

# Day 06
cargo run --bin day-06 src/bin/day-06-input.txt
cargo run --bin day-06 src/bin/day-06-input-part-2.txt
```

## Learnings

### Day 05

My original solution was working well with the example data (verified by Unit Tests).
However I was not able to generate a solution for the given puzzle input because my program was too slow. 

Until I added debug statements, I thought that my code hangs in an infinite loop. 
The code actually worked, it had just a horrible performance and ressource utilization. 
Once I saw the debug statements, I already realized that my code will run forever (which it didn't because it crashed, and VS Code too...).

My idea was to generate a `HashMap` for every Mapping table containing only the values that need to be mapped. 
When a mapping is needed, a `map.get(source)` call would return the next mapping value or - if nothing is returned - I knew I need to use the provided `source` value.

With Rust this was even super intuitive:

```rust
let next = *seed_to_soil.get(&seed).unwrap_or(&seed);
let next = *soild_to_fertilizer.get(&next).unwrap_or(&next);
let next = *fertilizer_to_water.get(&next).unwrap_or(&next);
```

This solution would return the destination very fast.
But, unfortunately, creating these `HashMaps` is what causes the massive performance problems.

Due to the large numbers, the `HashMap`s contained a massive number of records - from which most are not even needed...
For every line in the map a `HashMap` was generated containing as many elements as the `range` value was (_the value in the last column_).

```
seed-to-soil map:
2988689842 4194451945 100515351
2936009234 3353543976 52680608
588295233  2638661119 66434163
```

So just these three `HashMaps` above already contained `219.630.122` records...

When thinking again, I realized that the difference is just the `destination_range_start - source_range_start` value.
The new solution does not do any work upfront, everything is calculated on-demand. 
As a result, only required values are calculated which makes the program returning a result instantly. 
In addition way less memory is used because there is no need to store any individual mappings.
