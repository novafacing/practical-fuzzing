### Features

`libafl_targets` (and for that matter, most Rust crates) uses *features* to
conditionally compile parts of its functionality. For our uses, we need the feature
`sancov_8bit` because we'll be using 8-bit counter mode for `SanitizerCoverage`. We also
need the feature `observers` because we will be using a structure only defined with this
feature flag. You can read about features
[here](https://doc.rust-lang.org/cargo/reference/features.html).
