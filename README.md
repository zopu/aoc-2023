# Advent of Code 2023 (Rust)

Attempting to optimize performance towards all days running in sequence in under 1s total on my laptop (M1 MBP).

---

## Status as of day 16:

```
Day 1:  464us
Day 2:  185us
Day 3:  251us
Day 4:  274us
Day 5:  116us
Day 6:  14us
Day 7:  251us
Day 8:  354us
Day 9:  129us
Day 10: 187us
Day 11: 155us
Day 12: 1931us
Day 13: 176us
Day 14: 37ms
Day 15: 456us
Day 16: 1588us
Total time: 44ms. Remaining time budget: 955ms. 106ms/day avg remaining

% cargo build --release && hyperfine -N --warmup 10 './target/release/aoc-2023'
    Finished release [optimized] target(s) in 0.03s
Benchmark 1: ./target/release/aoc-2023
  Time (mean ± σ):      45.8 ms ±   2.7 ms    [User: 59.4 ms, System: 11.5 ms]
  Range (min … max):    41.0 ms …  56.1 ms    67 runs
```
