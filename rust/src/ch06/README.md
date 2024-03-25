# results

```sh
cargo run --release --bin ch06_compare
```

## num_playout=100

| name | win rate | time |
| ---- | -------- | ---- |
| monte_carlo num_playout 100 vs random | 100.00% | 2.22s |
| monte_carlo num_playout 100 vs 1 | 99.00% | 2.29s |
| mcts vs monte carlo 100 | 24.00% | 3.00s |
| mcts vs monte carlo 100 | 40.00% | 3.45s |
| mcts vs random 100 | 97.50% | 0.69s |
| mcts vs duct 100 | 59.00% | 2.09s |
| duct vs. monte carlo 100 | 29.00% | 2.92s |
| duct vs random 100 | 100.00% | 0.84s |

## num_playout=1000

| name | win rate | time |
| ---- | -------- | ---- |
| monte_carlo num_playout 1000 vs random | 100.00% | 21.42s |
| monte_carlo num_playout 1000 vs 10 | 92.00% | 22.10s |
| mcts vs monte carlo 1000 | 56.00% | 26.71s |
| mcts 2000 vs monte carlo 1000 | 69.00% | 32.67s |
| mcts vs random 1000 | 100.00% | 6.00s |
| mcts vs duct 1000 | 47.50% | 19.57s |
| duct vs. monte carlo 1000 | 59.00% | 28.57s |
| duct vs random 1000 | 100.00% | 7.79s |
