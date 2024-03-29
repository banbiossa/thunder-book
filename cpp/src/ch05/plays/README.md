# experiment

## monte carlo vs random

```sh
$ bazel build //src/ch05/plays:play_compare -c opt
$ ../../../bazel-bin/src/ch05/plays/play_compare
```

100 games

| action | win_rate| time |
| ------ | --------- | ---- |
| random vs random | 51.75% | 0.00s |
| mini max vs random | 98.25% | 30.16s |
| mini max vs. alpha-beta | 50.00% | 32.59s |
| iterative deepening 2ms vs. 1ms | 58.25% | 3.00s |
| monte carlo 3000 vs. random | 98.75% | 2.33s |
| monte carlo 3000 vs. monte carlo 30 | 65.00% | 2.27s |
| mcts 3000 vs monte carlo 3000 | 64.50% | 3.69s |
| mcts 3000 vs mcts 30 | 79.50% | 1.52s |
| thunder vs mcts 3000 | 56.75% | 2.14s |
| thunder vs mcts 1ms | 62.25% | 2.21s |
| thunder vs iterative deepening 1ms | 59.50% | 2.17s |
