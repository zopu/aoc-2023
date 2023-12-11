# Advent of Code 2023 (Rust)

Attempting to optimize performance towards all days running in sequence in under 1s total on my laptop (M1 MBP).

---

## Status as of day 11:

```
392us day 1 runtime
145us day 2 runtime
274us day 3 runtime
255us day 4 runtime
119us day 5 runtime
12us day 6 runtime
342us day 7 runtime
378us day 8 runtime
103us day 9 runtime
182us day 10 runtime
217us day 11 runtime
Total time: 2471us. Remaining time budget: 998ms. 43ms/day avg

% cargo build --release && hyperfine -N --warmup 10 './target/release/aoc-2023'
    Finished release [optimized] target(s) in 0.02s
Benchmark 1: ./target/release/aoc-2023
  Time (mean ± σ):       3.4 ms ±   0.2 ms    [User: 5.0 ms, System: 5.7 ms]
  Range (min … max):     3.1 ms …   4.3 ms    885 runs
```
