# results

```sh
cargo run --release --bin ch05_monte_carlo
```


| name | win_rate | time |
| ---- | -------- | ---- |
| monte_carlo num_playout 3000 vs 30 | 52.50% | 1.00s |
| monte_carlo num_playout 30 vs random | 85.25% | 0.01s |
| monte_carlo num_playout 3000 vs random | 86.75% | 0.99s |

```sh
cargo run --release --bin ch05_mcts
```

| name | win_rate | time |
| ---- | ----- | ---- |
| mcts num_playout 3000 vs 30 | 93.50% | 41.2s |
| mcts 3000 vs monte_carlo num_playout 3000 | 62.00% | 88.6s |
