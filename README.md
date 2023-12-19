# Advent of Code 2023 (Rust)

Attempting to optimize performance towards all days running in sequence in under 1s total on my laptop (M1 MBP).

---

## Status as of day 19:

```
Day 1:  599us
Day 2:  212us
Day 3:  316us
Day 4:  348us
Day 5:  134us
Day 6:  15us
Day 7:  385us
Day 8:  438us
Day 9:  118us
Day 10: 215us
Day 11: 124us
Day 12: 2376us
Day 13: 227us
Day 14: 17ms
Day 15: 476us
Day 16: 1905us
Day 17: 21ms
Day 18: 101us
Day 19: 414us
Total time: 46ms. Remaining time budget: 953ms. 158ms/day avg remaining

% cargo build --release && hyperfine -N --warmup 10 './target/release/aoc-2023'
    Finished release [optimized] target(s) in 0.03s
Benchmark 1: ./target/release/aoc-2023
  Time (mean ± σ):      44.1 ms ±   1.2 ms    [User: 101.0 ms, System: 12.9 ms]
  Range (min … max):    42.1 ms …  50.7 ms    66 runs
```
