# compare

duct は深く掘れば勝てる

100

| action | win_rate| time |
| ------ | --------- | ---- |
| random vs random | 57.00% | 0.00s |
| monte carlo 100 vs random | 100.00% | 1.46s |
| monte carlo 100 vs monte carlo 10 | 80.50% | 1.62s |
| mcts vs monte carlo 100 | 38.00% | 2.11s |
| mcts vs monte carlo 1000 | 65.00% | 20.56s |
| duct vs monte carlo 100 | 48.00% | 1.99s |
| duct vs monte carlo 1000 | 61.00% | 19.01s |
| mcts vs duct 100 | 50.00% | 1.12s |
| mcts vs duct 1000 | 47.50% | 10.59s |
| duct vs monte carlo 3000 | 80.00% | 56.37s |

1000

| action | win_rate| time |
| ------ | --------- | ---- |
| random vs random | 49.55% | 0.02s |
| monte carlo 100 vs random | 99.90% | 14.67s |
| monte carlo 100 vs monte carlo 10 | 83.90% | 16.59s |
| mcts vs monte carlo 100 | 39.15% | 20.61s |
| mcts vs monte carlo 1000 | 59.55% | 200.93s |
| duct vs monte carlo 100 | 36.20% | 56.99s |
| duct vs monte carlo 1000 | 63.30% | 1609.74s |
| mcts vs duct 100 | 52.10% | 11.10s |
| mcts vs duct 1000 | 47.70% | 2478.39s |
| duct vs monte carlo 3000 | 70.45% | 3238.18s |
