
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

改訂版. どっちがバグってそうだからテストを追加して様子見

| name | score | speed |
| ---- | -------- | ----- |
| normal | 137.74 | 46.35 ms |
| matrix | 137.80 | 24.22 ms |
| single | 137.80 | 25.45 ms |
