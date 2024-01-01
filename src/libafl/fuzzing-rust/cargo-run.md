# `cargo run`

Instead of having to run `cargo build` and then run your binary, you can use `cargo run`
as a shortcut. For example, to build and then run in release mode, we can run:


```sh
$ cargo run --release -- -h
Finished release [optimized] target(s) in 1.33s
     Running `target/release/first-fuzzer -h`
Usage: first-fuzzer --corpus <CORPUS> --solutions <SOLUTIONS>

Options:
  -c, --corpus <CORPUS>        Corpus directory
  -s, --solutions <SOLUTIONS>  Solutions directory
  -h, --help                   Print help
```

This is exactly the same as building then running, it just saves a step. Note the `--`
after the `cargo` arguments to separate arguments to `cargo` from arguments to our
binary.

From here out, we'll use `cargo run` whenever we want to run our fuzzer.
