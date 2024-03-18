
comparing bitset with normal

```sh
$ ../../bazel-bin/src/ch07/plays/time_beam_search_bitset | tee output.txt
```

the multi version is faster. no idea why.

| version | speed | % of normal |
| --- | --- | --- |
| normal | 47.3ms | 100% |
| multi |24.26ms | 51.2 %|
| single | 26.78ms| 56.6 %|
