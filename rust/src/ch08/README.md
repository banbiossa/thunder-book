# results


```sh
cargo run --release --bin ch08_compare
```

1000 games

| win % | time | name |
| ------- | ---- | ---- |
| 82.3% | 11.21s | mcts 1ms vs random |
| 49.0% | 0.09s | random vs random |
| 91.8% | 14.54s | bitset mcts 1ms vs random |
| 50.0% | 0.01s | bitset random vs random |
| 69.3% | 31.94s | bitstate mcts vs normal mcts 1ms |
