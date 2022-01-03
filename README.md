# Linear and binary search implementations in Rust.

## Why?
I was bored.

## Benchmarks
- Set the list size and seed in lib.rs.
- Change the seed to test with different targets. We need the target to be the same for both functions. At the same time, we need to be able to test with different targets. This is where being able to change the seed comes in handy.
- Run this
    ```sh
    cargo bench -- --nocapture
    ```
    Note: You need Rust nightly to run benchmarks.
    To switch to nightly for the current directory, run
    ```sh
    rustup override set nightly
    ```

#### Well, that was kinda fun.