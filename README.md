# Advent of Code 2023 (Rust)

Attempting to optimize performance towards all days running in sequence in under 1s total on my laptop (M1 MBP).

---

## Status as of day 13:

```
Day 1:  548us
Day 2:  139us
Day 3:  291us
Day 4:  244us
Day 5:  115us
Day 6:  11us
Day 7:  327us
Day 8:  385us
Day 9:  108us
Day 10: 167us
Day 11: 107us
Day 12: 2159us
Day 13: 230us
Total time: 4909us. Remaining time budget: 995ms. 43ms/day avg remaining

% cargo build --release && hyperfine -N --warmup 10 './target/release/aoc-2023'
    Finished release [optimized] target(s) in 0.05s
Benchmark 1: ./target/release/aoc-2023
  Time (mean ± σ):       5.6 ms ±   0.2 ms    [User: 11.1 ms, System: 8.9 ms]
  Range (min … max):     5.2 ms …   6.7 ms    507 runs
```
