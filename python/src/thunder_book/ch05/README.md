# results

```sh
python monte_carlo.py
```

|name|score|time|
|----|-----|----|
|10 vs. 3|0.66|0.52 s|
|300 vs. random|0.97|11.17 s|


```sh
python monte_carlo_tree_search.py
```

|name|score|time|
|----|-----|----|
| mcts 100 vs. 10 | 80.00% | 4.70s |
| mcts vs. monte carlo 3000 | 63.75% | 224.01s |

```sh
python thunder_search.py
```

|name|score|time|
|----|-----|----|
| thunder_search vs. mcts 1ms | 57.50% | 2.25s |
| thunder_search vs. mcts 100 playout | 56.75% | 7.85s |
| thunder_search vs. iterative deepening 1ms | 89.75% | 2.19s |
