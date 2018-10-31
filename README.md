# AOS vs SOA

Naive benchmark to show how the *struct of arrays* data layout benefits from caching to improve performance.

Sample:

```
cargo bench
```

Sample output:

```
running 2 tests
test tests::array_of_structs::avg_age ... bench:   2,427,309 ns/iter (+/- 201,343)
test tests::struct_of_arrays::avg_age ... bench:   2,192,988 ns/iter (+/- 136,117)

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured; 0 filtered out
```

If you have [`cargo benchcmp`](https://github.com/BurntSushi/cargo-benchcmp) installed, you can also compare results:

```bash
for i in $(seq 1 10); do cargo bench > benchmark-output$i; done
for i in $(seq 1 10)
do output="$(cargo benchcmp tests::array_of_structs tests::struct_of_arrays benchmark-output$i)"
  if [[ $i -eq 1 ]]
  then echo "${output}"
  else echo "${output}" | tail -1
  fi
done
```

Sample output:

```
 name       tests::array_of_structs ns/iter  tests::struct_of_arrays ns/iter  diff ns/iter  diff %  speedup
 ::avg_age  2,427,309                        2,192,988                            -234,321  -9.65%   x 1.11
 ::avg_age  2,438,766                        2,215,783                            -222,983  -9.14%   x 1.10
 ::avg_age  2,444,267                        2,196,445                            -247,822  -10.14%   x 1.11
 ::avg_age  2,430,973                        2,195,141                            -235,832  -9.70%   x 1.11
 ::avg_age  2,462,707                        2,185,442                            -277,265  -11.26%   x 1.13
 ::avg_age  2,414,035                        2,211,378                            -202,657  -8.39%   x 1.09
 ::avg_age  2,431,339                        2,219,773                            -211,566  -8.70%   x 1.10
 ::avg_age  2,433,808                        2,225,186                            -208,622  -8.57%   x 1.09
 ::avg_age  2,404,514                        2,175,141                            -229,373  -9.54%   x 1.11
 ::avg_age  2,409,768                        2,200,761                            -209,007  -8.67%   x 1.09
```
