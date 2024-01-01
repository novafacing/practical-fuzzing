# `cargo build`

When we run `cargo build`, we have seen that `cargo` will:

- Run our build script
- Build (compile) our crate

When we build an executable like our fuzzer with `cargo build`, `cargo` will put the
resulting binary in `target/debug/<EXECUTABLE_NAME>`. Let's go ahead and build our
fuzzer:

```sh
$ cargo build
$ ls target/debug/
build  deps  examples  first-fuzzer  first-fuzzer.d  incremental
```

We can run our `first-fuzzer` binary:

```sh
$ ./target/debug/first-fuzzer -h
Usage: first-fuzzer --corpus <CORPUS> --solutions <SOLUTIONS>

Options:
  -c, --corpus <CORPUS>        Corpus directory
  -s, --solutions <SOLUTIONS>  Solutions directory
  -h, --help                   Print help
```

If we build in the `release` profile with `cargo build --release`, instead of being
located in the `debug` subdirectory, our binary will be in the `release` subdirectory.
Generally, you'll use either the `debug` (default) profile or the `release` profile, and
you should always use the `release` profile when running real fuzzing campaigns.
Otherwise, you are leaving performance on the table!

We can build and run just the same with the `release` profile:

```sh
$ cargo build --release
$ ./target/release/first-fuzzer -h
Finished release [optimized] target(s) in 1.33s
     Running `target/release/first-fuzzer -h`
Usage: first-fuzzer --corpus <CORPUS> --solutions <SOLUTIONS>

Options:
  -c, --corpus <CORPUS>        Corpus directory
  -s, --solutions <SOLUTIONS>  Solutions directory
  -h, --help                   Print help
```
