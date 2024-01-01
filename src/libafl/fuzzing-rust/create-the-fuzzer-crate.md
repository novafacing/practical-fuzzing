## Create The Fuzzer Crate

Navigate back to the directory containing `fuzz-target`. Then, create a fuzzer crate and
navigate to it, then run it to make sure everything is set up correctly:

```sh
$ cargo new --bin first-fuzzer
$ cd first-fuzzer
$ cargo run --bin first-fuzzer
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/first-fuzzer`
Hello, world!
```
