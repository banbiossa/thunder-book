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
> Score:  201, time:      0.15
> greedy
> Score:  512, time:      0.52
> beam_search width 2
> Score:  643, time:      40.43
> beam_search width 5 with 1ms
> Score:  655, time:      10.54
> beam_search width 5 with 10ms
> Score:  678, time:      76.43
```
