# 実験

```sh
python monte_carlo.py
```

| name | score | time |
| ---- | ----- | ---- |
| monte carlo 10 vs random | 95.50% | 2.79 |

```sh
python mcts.py
```

| name | score | time |
| ---- | ----- | ---- |
| mcts vs random | 99.00% | 8.49 |
| mcts vs monte carlo 100 | 30.50% | 31.92 |
| mcts vs monte carlo 3000 | 67.00% | 940.26 |


```sh
python duct.py
```
| name | score | time |
| ---- | ----- | ---- |
| duct vs mcts 1000 | 0.49 | 227.83 |
| duct vs monte carlo 500 | 0.58 | 201.90 |
