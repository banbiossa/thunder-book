# compare

duct は深く掘れば勝てる

mcts は多分バグってる

| action | win_rate| time |
| ------ | --------- | ---- |
| random vs random | 48.50% | 0.00s |
| monte carlo 100 vs random | 100.00% | 1.45s |
| monte carlo 100 vs monte carlo 10 | 84.50% | 1.58s |
| mcts vs monte carlo 100 | 0.00% | 2.06s |
| duct vs monte carlo 100 | 31.00% | 1.92s |
| duct vs monte carlo 1000 | 62.50% | 18.63s |
| duct vs monte carlo 3000 | 66.50% | 55.95s |
