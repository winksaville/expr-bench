# expr-bench

Experiment with benchmarking on rust. The builtin benchmark
code doesn't seem very good and [Criterion](https://github.com/bheisler/criterion.rs)
seems the way to.

## Building and running

```
wink@3900x 22-05-11T18:42:08.146Z:~/prgs/rust/myrepos/expr-bench (main)
$ cargo criterion
    Finished bench [optimized] target(s) in 0.02s
  Executable benches src/lib.rs (target/release/deps/expr_bench-363a72fe10cc2c88)
  Executable benches/bench.rs (target/release/deps/bench-eed321211b3dce9d)
Gnuplot not found, using plotters backend
add_two                 time:   [1.1091 ns 1.1107 ns 1.1125 ns]                     
                        change: [-1.3374% -0.9972% -0.6471%] (p = 0.00 < 0.05)
                        Change within noise threshold.

add_two_twice           time:   [2.0643 ns 2.0656 ns 2.0668 ns]                           
                        change: [+0.3594% +0.4711% +0.5848%] (p = 0.00 < 0.05)
                        Change within noise threshold.

add_two_stack           time:   [1.3432 ns 1.3508 ns 1.3594 ns]                           
                        change: [-2.0887% -1.7371% -1.3588%] (p = 0.00 < 0.05)
                        Performance has improved.


running 2 tests
test tests::it_works ... ignored
test tests::bench_add_two ... bench:           0 ns/iter (+/- 0)

test result: ok. 0 passed; 0 failed; 1 ignored; 1 measured; 0 filtered out; finished in 4.05s

wink@3900x 22-05-11T18:42:47.125Z:~/prgs/rust/myrepos/expr-bench (main)
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
