# Add Additional Dependencies

We'll use another two dependencies in our fuzzer:

- `clap`, for command-line argument parsing. We'll use the `derive` feature to make
  creating our argument parser super easy.
- `mimalloc`, to allow our fuzzer to use a separate allocator from our target.

Add them by running:


```sh
$ cargo add clap --features=derive
$ cargo add mimalloc
```
