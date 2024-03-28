
- `bazel test --test_output=errors test/ch03:maze-state-test` で test

```sh
$ ./bazel-bin/src/ch03/score-average
```

30*30-100, using consts

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

30*30-100, using params (struct), vec

```
random
Score:  207.64, time    0.005
greedy
Score:  489.2, time     0.084
beam width 2
Score:  632.07, time    17.75
beam width 5 with timekeeper 1ms
Score:  679.1, time     9.825
beam width 5 with timekeeper 10ms
Score:  681.33, time    42.996
chokudai search with timekeeper 1ms
Score:  641.97, time    15.663
chokudai search with timekeeper 10ms
Score:  701, time       136.057
```
