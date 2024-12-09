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

Then run the benchmarks

```
cargo bench
```

## Usage

````
rust-dev $ cargo bench
   Compiling rust-dev v0.1.0 (C:\Users\westd\OneDrive\Desktop\development\rust-dev)
    Finished `bench` profile [optimized] target(s) in 0.76s
     Running unittests src/main.rs (target\release\deps\rust_dev-b3651944c9246887.exe)

running 4 tests
test test::bench_creating_cow    ... bench:           1.81 ns/iter (+/- 0.12)
test test::bench_creating_string ... bench:          83.84 ns/iter (+/- 25.33)
test test::mutating_cow          ... bench:         267.57 ns/iter (+/- 20.64)
test test::mutating_owned_string ... bench:         264.92 ns/iter (+/- 19.09)

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured; 0 filtered out; finished in 6.93s
o```
````