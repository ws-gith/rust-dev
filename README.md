# Cow vs String Benchmark

This is a simple code example to compare the overhead between String and Cow, in general usage and during mutation.

## Install

First clone the repo

```
git clone https://github.com/ws-gith/rust-dev.git
```

Then `cd` in to the directory

```
cd rust-dev
```

Make sure you're on the nightly build of rust

```
rustup default nightly
```

Then run the benchmarks

```
cargo bench
```

## Usage

````
rust-dev $ cargo bench
    Finished `bench` profile [optimized] target(s) in 0.02s
     Running unittests src/main.rs (target\release\deps\rust_dev-b3651944c9246887.exe)

running 4 tests
test test::bench_creating_cow    ... bench:           1.81 ns/iter (+/- 0.35)
test test::bench_creating_string ... bench:          93.62 ns/iter (+/- 12.34)
test test::mutating_cow          ... bench:         350.50 ns/iter (+/- 59.31)
test test::mutating_owned_string ... bench:         348.64 ns/iter (+/- 93.63)

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured; 0 filtered out; finished in 7.17s
o```
````