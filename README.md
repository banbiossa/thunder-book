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
> Score:  205.53, time    0.03
> greedy
> Score:  517.8, time     0.023
> beam width 2
> Score:  640.53, time    4.498
> beam width 5 with timekeeper 1ms
> Score:  686.5, time     8.236
> beam width 5 with timekeeper 10ms
> Score:  686.77, time    14.583
```

```
$ cd python
$ python src/thunder_book/ch03/score_average.py
> play games=100
> random
> Score:  201, time:      0.13
> greedy
> Score:  512, time:      0.53
> beam_search
> Score:  643, time:      40.55
> beam_search with 1ms
> Score:  661, time:      10.56
> beam_search with 10ms
> Score:  678, time:      76.93
```
