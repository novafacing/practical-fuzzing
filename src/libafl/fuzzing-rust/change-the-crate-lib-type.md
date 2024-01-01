# Change The Crate Lib Type

Before we move on, we need to change the crate type of our fuzz target's crate. Rust's
build system supports many crate types, which you can read more about
[here](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-crate-type-field)
. Add these two lines to your `Cargo.toml` file:

```toml
[lib]
crate-type = ["staticlib"]
```

For most normal rust libraries or crates, this is not necessary. However because we will
be instrumenting this library for coverage feedback, we will need to build it separately
and link it into our fuzzer. We'll discuss this further in the fuzzer section.

After changing the crate type, run `cargo build` and `cargo test` again to be sure
everything is still working as expected.
