on a 8GB machine

## pypy-3.10

```
$ uv run ./src/thunder_book/ch03/score_average.py
```

| name | score | time |
| ------ | ----- | ---- |
| random | 201 | 0.58 |
| greedy | 512 | 0.95 |
| beam_search_width_2 | 643 | 66.01 |
| beam_search_width_5_with_1ms | 648 | 14.74 |
| beam_search_width_5_with_10ms | 677 | 91.94 |
| chokudai_search_width_1_with_1ms | 438 | 57.42 |
| chokudai_search_width_1_with_10ms | 625 | 137.68 |

## python-3.12

```
$ uv run ./src/thunder_book/ch03/score_average.py
```

| name | score | time |
| ------ | ----- | ---- |
| random | 201 | 0.13 |
| greedy | 512 | 0.56 |
| beam_search_width_2 | 643 | 45.61 |
| beam_search_width_5_with_1ms | 656 | 11.18 |
| beam_search_width_5_with_10ms | 678 | 78.91 |
| chokudai_search_width_1_with_1ms | 440 | 39.47 |
| chokudai_search_width_1_with_10ms | 647 | 120.06 |


## 32GB machine

the below was on a 32GB machine

```
$ python src/thunder_book/ch03/score_average.py
```

| name | score | time |
| ------ | ----- | ---- |
| random | 201 | 0.19 |
| greedy | 512 | 0.61 |
| beam_search_width_2 | 643 | 49.40 |
| beam_search_width_5_with_1ms | 659 | 10.68 |
| beam_search_width_5_with_10ms | 678 | 81.39 |
| chokudai_search_width_1_with_1ms | 432 | 44.07 |
| chokudai_search_width_1_with_10ms | 642 | 610.30 |
