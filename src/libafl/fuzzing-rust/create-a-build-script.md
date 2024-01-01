# Create a Build Script

`cargo` supports *build scripts* (you can read the documentation on them
[here](https://doc.rust-lang.org/cargo/reference/build-scripts.html)). By creating a
`build.rs` file in our crate directory, `cargo` will run it before compiling the crate.

`build.rs` scripts provide a few key features, notably the ability to modify or add
linking information during crate compilation. We need our build script to do two things:

1. Compile `first-target` with the coverage sanitizer enabled
2. Tell `cargo` (and through it, `rustc`) to link with the coverage sanitized library
   when compiling the fuzzer binary
