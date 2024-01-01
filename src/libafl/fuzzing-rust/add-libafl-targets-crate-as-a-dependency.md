# Add LibAFL-Targets Crate as a Dependency

The `libafl_targets` crate provides target-specific functionality and *runtimes* for
various instrumentation mechanisms designed to be used with `libafl`. In our case, it
provides an easy way to interact with `SanitizerCoverage`. We'll go in depth on our
instrumentation [later](#coverage-sanitizer), for now we'll just add the crate to our
`Cargo.toml`:

```toml
libafl_targets = { version = "0.11.2", features = [
    "sancov_8bit",
    "observers",
] }
```