# results

```sh
cargo run --release --bin ch05_compare
```


|name|win_rate%|time|
|----|-----|----|
| mini max depth 10 vs. random | 99.00% | 38.05s |
| random vs random | 49.00% | 0.00s |
| mini max depth 10 vs. mini max depth 10 | 50.00% | 77.78s |
| mini max depth 10 vs. alpha-beta depth 10 | 50.00% | 42.43s |
| iterative deepening 1ms vs. 1ms | 50.50% | 2.60s |
| iterative deepening 2ms vs. 1ms | 54.25% | 3.75s |
| iterative deepening 5ms vs. 1ms | 55.75% | 7.14s |
| monte_carlo num_playout 3000 vs random | 98.50% | 3.82s |
| monte_carlo num_playout 30 vs random | 97.00% | 0.04s |
| monte_carlo num_playout 3000 vs 30 | 68.00% | 3.84s |
| mcts 3000 vs monte_carlo num_playout 3000 | 63.25% | 5.44s |
| mcts num_playout 3000 vs 30 | 78.25% | 1.64s |
| thunder vs. mcts num_playout 3000 | 53.75% | 2.34s |
| thunder vs. mcts 1ms | 59.25% | 2.27s |
| thunder vs. iterative deepening 1ms | 55.75% | 2.53s |
