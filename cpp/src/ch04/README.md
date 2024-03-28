# results

```sh
$ bazel build //src/ch04:score_average -c
$ ../../bazel-bin/src/ch04/score_average
```

using consts (10 times faster than rust)

```
play 100 games with 10000 simulations
random hill_climb simulated_annealing 

random: 71.69 elpased: 0ms
hill_climb:     96.3 elpased: 102ms
simulated_annealing:    97.94 elpased: 127ms
```

