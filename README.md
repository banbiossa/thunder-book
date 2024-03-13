# thunder-book

thunder book の実装. c++/python/rust でやってみる

- cpp
- python
- rust

# comparisons

## chapter03 cpp vs. python

```
$ cd cpp
$ bazel build //src/ch03:score-average -c opt
$ ./bazel-bin/src/ch03/score-average 
> random
> Score:  207.64, time    0.004
> greedy
> Score:  489.2, time     0.008
> beam width 2
> Score:  632.07, time    1.395
> beam width 5 with timekeeper 1ms
> Score:  681.33, time    6.042
> beam width 5 with timekeeper 10ms
> Score:  681.33, time    6.588
> chokudai search with timekeeper 1ms
> Score:  687.89, time    11.533
> chokudai search with timekeeper 10ms
> Score:  710.02, time    109.865
```

```
$ cd python
$ python src/thunder_book/ch03/score_average.py
play games=100
> random
> Score:  201, time:      0.13
> greedy
> Score:  512, time:      0.52
> beam_search width 2
> Score:  643, time:      39.87
> beam_search width 5 with 1ms
> Score:  663, time:      10.47
> beam_search width 5 with 10ms
> Score:  678, time:      76.48
> chokudai search width 1 with 1ms
> Score:  445, time:      35.51
> chokudai search width 1 with 10ms
> Score:  650, time:      117.35
```

```
$ cd rust
$ cargo run --release --bin ch03_compare
> name:	random
> score:	198.19 time:	0.01
> name:	greedy
> score:	499.53 time:	0.09
> name:	beam_search - width: 2, depth: 100
> score:	616.6 time:	6.59
> name:	beam search - width: 5, time: 1ms
> score:	686.89 time:	8.69
> name:	beam search - width: 5, time: 10ms
> score:	686.95 time:	16.46
> name:	chokudai search - width: 1, 2 beams
> score:	630.24 time:	8.10
> name:	chokudai search - width: 1, 1ms
> score:	678.93 time:	21.21
> name:   chokudai search - width: 1, 10ms
> score:  706.95 time:    204.42
```
