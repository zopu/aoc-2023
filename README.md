# Advent of Code 2023 (Rust)

Attempting to optimize performance towards all days running in sequence in under 1s total on my laptop (M1 MBP).

---

## Status as of day 9:

```
515us day 1 runtime
176us day 2 runtime
291us day 3 runtime
234us day 4 runtime
128us day 5 runtime
13us day 6 runtime
404us day 7 runtime
428us day 8 runtime
118us day 9 runtime
Total time: 2387us. Remaining time budget: 998ms. 43ms/day avg

% cargo build --release && hyperfine -N --warmup 10 './target/release/aoc-2023'
Finished release [optimized] target(s) in 0.02s
Benchmark 1: ./target/release/aoc-2023
Time (mean ± σ): 3.1 ms ± 0.2 ms [User: 4.8 ms, System: 5.9 ms]
Range (min … max): 2.7 ms … 3.9 ms 943 runs
```
