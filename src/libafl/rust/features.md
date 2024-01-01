
# Features

Most Rust crates use *features* to conditionally compile parts of its functionality.
Features are enabled in the [Cargo.toml](library-crates/cargotoml.md) file. For example,
to enable the `logging` feature for `regex`:

```toml
[dependencies]
regex = { version = "1.8.1", features = ["logging"] }
```

Features can also be added when `cargo add`-ing a dependency to a crate like:

```sh
cargo add regex --features=logging
```

See the [Features page](https://doc.rust-lang.org/cargo/reference/features.html) for a
complete explanation of how features can be used.