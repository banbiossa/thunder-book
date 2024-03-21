# results


```sh
cargo run --release --bin ch08_compare | tee output.md
```

| win % | time | name |
| ------- | ---- | ---- |
| 82.5% | 1.14s | mcts 1ms vs random |
| 46.0% | 0.01s | random vs random |
| 94.0% | 1.47s | bitset mcts 1ms vs random |
| 56.0% | 0.00s | bitset random vs random |
| 75.8% | 3.26s | bitstate vs normal 1ms |
