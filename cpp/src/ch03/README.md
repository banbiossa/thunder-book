
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

30*30, using params (struct)

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
