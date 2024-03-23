# results

```sh
cargo run --release --bin ch05_monte_carlo
```


| name | win_rate | time |
| ---- | -------- | ---- |
| monte_carlo num_playout 3000 vs 30 | 52.50% | 1.00s |
| monte_carlo num_playout 30 vs random | 85.25% | 0.01s |
| monte_carlo num_playout 3000 vs random | 86.75% | 0.99s |
