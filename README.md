# thunder-book

thunder book の実装. c++ と python でやってみる

- cpp
- python

# comparisons

## chapter03 cpp vs. python

```
$ cd cpp
$ bazel build //src/ch03:score-average
$ ./bazel-bin/src/ch03/score-average 
> random
> Score:  205.53, time    0.032
> greedy
> Score:  517.8, time     0.023
> beam
> Score:  640.53, time    4.499
> beam with timekeeper 1ms
> Score:  686.39, time    8.268
> beam with timekeeper 10ms
> Score:  686.77, time    14.643
```

```
$ cd python
$ python src/thunder_book/ch03/score_average.py
> play games=100
> random
> Score:  201, time:      0.14
> greedy
> Score:  512, time:      0.54
> beam_search
> Score:  643, time:      41.12
> beam_search with 1ms
> Score:  643, time:      9.85
> beam_search with 10ms
> Score:  643, time:      41.08
```
