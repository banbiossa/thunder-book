
- `bazel test --test_output=errors test/ch03:maze-state-test` で test

```sh
$ ./bazel-bin/src/ch03/score-average
```

30*30, using consts

```
random
Score:  207.64, time    0.001
greedy
Score:  489.2, time     0.003
beam width 2
Score:  632.07, time    1.373
beam width 5 with timekeeper 1ms
Score:  681.33, time    6.068
beam width 5 with timekeeper 10ms
Score:  681.33, time    6.422
chokudai search with timekeeper 1ms
Score:  685.49, time    11.5
chokudai search with timekeeper 10ms
Score:  709.94, time    109.792
```

30*30, using params (struct), shared_ptr, array

```
random
Score:  207.64, time    0.005
greedy
Score:  489.2, time     0.087
beam width 2
Score:  141.21, time    6.409
beam width 5 with timekeeper 1ms
Score:  160.18, time    8.698
beam width 5 with timekeeper 10ms
Score:  152.76, time    59.436
chokudai search with timekeeper 1ms
Score:  231.04, time    18.765
chokudai search with timekeeper 10ms
Score:  257.64, time    201.708
```

30*30, using params (struct), copy for beam search

```
random
Score:  207.64, time    0.004
greedy
Score:  489.2, time     0.08
beam width 2
Score:  632.07, time    18.077
beam width 5 with timekeeper 1ms
Score:  183.89, time    8.805
beam width 5 with timekeeper 10ms
Score:  169.87, time    60.861
chokudai search with timekeeper 1ms
Score:  245.86, time    18.929
chokudai search with timekeeper 10ms
Score:  227, time       200.408
```

30*30 using params, shared_ptr, vec

```
random
Score:  207.64, time    0.003
greedy
Score:  489.2, time     0.088
beam width 2
Score:  148.85, time    9.097
beam width 5 with timekeeper 1ms
Score:  158.17, time    8.732
beam width 5 with timekeeper 10ms
```
