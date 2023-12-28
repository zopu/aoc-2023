# Advent of Code 2023 (Rust)

Attempting to optimize performance towards all days running in sequence in under 1s total on my laptop (M1 MBP).

---

## Current status:

```
Day 1:  559us
Day 2:  132us
Day 3:  321us
Day 4:  314us
Day 5:  116us
Day 6:  13us
Day 7:  329us
Day 8:  361us
Day 9:  103us
Day 10: 160us
Day 11: 103us
Day 12: 1861us
Day 13: 201us
Day 14: 13ms
Day 15: 448us
Day 16: 1491us
Day 17: 20ms
Day 18: 116us
Day 19: 395us
Day 20: 626us
Day 21: 372us
Day 22: 8ms
Day 23: 132ms
Day 24: 989us
Day 25: 197ms
Total time: 381ms. Remaining time budget: 618ms.

% cargo build --release && hyperfine -N --warmup 10 './target/release/aoc-2023'
    Finished release [optimized] target(s) in 0.03s
Benchmark 1: ./target/release/aoc-2023
  Time (mean ± σ):     383.3 ms ±   1.9 ms    [User: 407.8 ms, System: 12.5 ms]
  Range (min … max):   380.6 ms … 386.2 ms    10 runs
```
