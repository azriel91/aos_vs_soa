# AOS vs SOA

Naive benchmark to show how the *struct of arrays* data layout benefits from caching to improve performance.

Sample:

```
cargo bench
```

Sample output:

```
running 2 tests
test tests::array_of_structs::avg_age ... bench:   2,526,944 ns/iter (+/- 959,621)
test tests::struct_of_arrays::avg_age ... bench:   2,214,163 ns/iter (+/- 223,417)

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured; 0 filtered out
```

If you have [`cargo benchcmp`](https://github.com/BurntSushi/cargo-benchcmp) installed, you can also compare results:

```bash
for i in $(seq 1 10); do cargo bench > benchmark-output$i; done
for i in $(seq 1 10); do cargo benchcmp tests::array_of_structs tests::struct_of_arrays benchmark-output$i; done
```

Sample output:

```
 name       tests::array_of_structs ns/iter  tests::struct_of_arrays ns/iter  diff ns/iter   diff %  speedup
 ::avg_age  2,526,944                        2,214,163                            -312,781  -12.38%   x 1.14
 name       tests::array_of_structs ns/iter  tests::struct_of_arrays ns/iter  diff ns/iter  diff %  speedup
 ::avg_age  2,436,020                        2,194,025                            -241,995  -9.93%   x 1.11
 name       tests::array_of_structs ns/iter  tests::struct_of_arrays ns/iter  diff ns/iter  diff %  speedup
 ::avg_age  2,424,791                        2,231,951                            -192,840  -7.95%   x 1.09
 name       tests::array_of_structs ns/iter  tests::struct_of_arrays ns/iter  diff ns/iter  diff %  speedup
 ::avg_age  2,436,198                        2,200,000                            -236,198  -9.70%   x 1.11
 name       tests::array_of_structs ns/iter  tests::struct_of_arrays ns/iter  diff ns/iter   diff %  speedup
 ::avg_age  2,438,697                        2,182,015                            -256,682  -10.53%   x 1.12
 name       tests::array_of_structs ns/iter  tests::struct_of_arrays ns/iter  diff ns/iter  diff %  speedup
 ::avg_age  2,434,677                        2,204,860                            -229,817  -9.44%   x 1.10
 name       tests::array_of_structs ns/iter  tests::struct_of_arrays ns/iter  diff ns/iter  diff %  speedup
 ::avg_age  2,425,235                        2,195,862                            -229,373  -9.46%   x 1.10
 name       tests::array_of_structs ns/iter  tests::struct_of_arrays ns/iter  diff ns/iter  diff %  speedup
 ::avg_age  2,422,223                        2,186,381                            -235,842  -9.74%   x 1.11
 name       tests::array_of_structs ns/iter  tests::struct_of_arrays ns/iter  diff ns/iter  diff %  speedup
 ::avg_age  2,425,502                        2,211,902                            -213,600  -8.81%   x 1.10
 name       tests::array_of_structs ns/iter  tests::struct_of_arrays ns/iter  diff ns/iter  diff %  speedup
 ::avg_age  2,434,509                        2,213,452                            -221,057  -9.08%   x 1.10
```
