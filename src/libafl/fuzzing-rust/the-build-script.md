# The Build Script

To pass arguments to the compilation of a specific dependency, we can use the
`cargo rustc` subprogram. This lets us pass arguments to `rustc`, the Rust compiler, as
well as to [*LLVM*](https://llvm.org), which `rustc` uses for final code generation and
additional optimization. We'll use `std::process::Command`, which is similar to using
`subprocess.Popen` in Python or `system` in C, although much more flexible.

We'll walk through the `cargo rustc` arguments that we'll add in addition to the
`SanitizerCoverage` flags.

- `-p first-target-solution`: Tell `rustc` to just build the target with these arguments,
  rather than pass these arguments to *all* the code we're compiling.
- `--target-dir target/first-target/`: Tell `rustc` to build in a subdirectory of the
  default `target` directory, to avoid conflicting with the currently running build
  process' lock on the `target` directory.
- `link-dead-code`: Don't prune any unused code from the compilation result.
- `lto=no`: Don't do link-time optimization, because it might remove some code or
   instrumentation.
- `--emit=dep-info,link`: Emit linking and dependency information.
- `opt-level={}`: We just pass the opt level we are compiling the fuzzer at to the
  target as well. This allows us to use `--release`.


```rust
use std::{env::var, process::Command};

fn main() {
    let status = Command::new("cargo")
        .arg("rustc")
        .arg("-p")
        .arg("first-target")
        .arg("--target-dir")
        .arg("target/first-target/")
        .arg("--")
        .arg("-C")
        .arg(format!(
            "opt-level={}",
            var("OPT_LEVEL").expect("No OPT_LEVEL variable set")
        ))
        .arg("-C")
        .arg("prefer-dynamic")
        .arg("-C")
        .arg("passes=sancov-module")
        .arg("-C")
        .arg("llvm-args=-sanitizer-coverage-level=3")
        .arg("-C")
        .arg("llvm-args=-sanitizer-coverage-inline-8bit-counters")
        .arg("-C")
        .arg("llvm-args=-sanitizer-coverage-prune-blocks=0")
        .arg("-C")
        .arg("link-dead-code")
        .arg("-C")
        .arg("lto=no")
        .arg("--emit=dep-info,link")
        .status()
        .expect("Failed to spawn Cargo");

    assert!(status.success(), "Target build command failed");

```

We'll also use some specially formatted print statements in our build script to instruct
`cargo` to link with the library. Note the `env!` macro, which grabs the value of an
environment variable at compile time (and is a compile error if the variable isn't
present). The `CARGO_MANIFEST_DIR` environment variable is set automatically by `cargo`
during build script compilation and execution. You can check the full list of these
environment variables
[here](https://doc.rust-lang.org/cargo/reference/environment-variables.html). You can
also check out the full list of special print messages
[here](https://doc.rust-lang.org/cargo/reference/build-scripts.html).

```rust
    println!(
        "cargo:rustc-link-search={}/target/first-target/debug/",
        env!("CARGO_MANIFEST_DIR")
    );
    println!("cargo:rustc-link-lib=static=first_target");
}
```
