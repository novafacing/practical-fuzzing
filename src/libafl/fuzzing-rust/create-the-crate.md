# Create The Crate

Create the crate and enter the directory for our first fuzzer with:

```sh
$ cargo new --lib first-target
$ cd first-target
```

You should have a template Library crate as we saw in [the Rust
Basics](./RustBasics.md#creating-a-binary-crate). To make sure everything is working
correctly, we'll run the tests `cargo` gives us 

```sh
$ cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.13s
     Running unittests src/lib.rs (target/debug/deps/first_target-d654f36012dfaf5d)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests first-target

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

If you see that, we're ready to go!
