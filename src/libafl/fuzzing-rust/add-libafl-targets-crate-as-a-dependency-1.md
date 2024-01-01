# Add LibAFL-Targets Crate as a Dependency

We need to utilize data structures from `libafl_targets` to interpret the coverage
information being gathered by the instrumentation. To do that, we'll depend on the
crate the same way we did in our target.

We can add `libafl_targets`  with the `sancov_8bit` and `observers` feature to our crate
by adding this line to our `Cargo.toml` file under the `[dependencies]` section:

```toml
libafl_targets = { version = "0.11.2", features = [
    "sancov_8bit",
    "observers",
] }
```

