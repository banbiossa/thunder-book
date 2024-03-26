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

