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


```
$ cd rust
$ cargo run --release --bin ch03_compare
```

| name | score | time |
| ---- | ----- | ---- |
| random | 200.83 | 0.01 |
| greedy | 499.53 | 0.07 |
| beam_search - width: 2, depth: 100  | 616.6 | 6.74 |
| beam search - width: 5, time: 1ms | 686.76 | 8.73 |
| beam search - width: 5, time: 10ms | 686.95 | 16.73 |
| chokudai search - width: 1, 2 beams | 630.24 | 8.50 |
| chokudai search - width: 1, 1ms | 677.47 | 21.81 |
| chokudai search - width: 1, 10ms | 706.69 | 206.42 |
