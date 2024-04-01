# compare

bitset は壊れてそう. なぜか先手が必ず勝つようになっている。

| action | win_rate| time |
| ------ | --------- | ---- |
| mcts bitset 1ms vs random | 48.50% | 1.06s |
| mcts bitset 1ms vs mcts normal 1ms | 50.00% | 3.35s |
| random vs random | 48.50% | 0.10s |
| mcts 1ms vs random | 83.00% | 1.33s |
| mcts 10 vs random | 80.50% | 2.05s |
