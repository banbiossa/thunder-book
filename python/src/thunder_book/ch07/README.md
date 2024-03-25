# chap07

```python
python beam_search.py
```

the results are as below. at least in python-land, the numpy optimization works.

| version | speed | % of normal |
| ------| ------ | -------- |
| numpy | 1,172 ms | 84.6 % |
| single | 1,781 ms | 128.6 % |
| multi | 1,685 ms | 121.7 % |
| normal | 1,385 ms | 100.0% |


## scores

| name | score | time |
| ---- | ----- | ---- |
| beam numpy zobrist True  | 141.50 | 7.37 |
| beam single zobrist True  | 141.50 | 10.38 |
| beam multi zobrist True  | 141.50 | 10.82 |
| beam normal zobrist False  | 141.20 | 15.24 |
| beam normal zobrist True  | 141.40 | 9.71 |
