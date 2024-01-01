# Fuzzer

Finally, we'll create the [`Fuzzer`](https://docs.rs/libafl/latest/libafl/fuzzer/trait.Fuzzer.html).

The fuzzer is the frontend to execution and kicks everything off.

Add the `use` declaration for it (and the `Fuzzer` trait):

```rust
use libafl::{StdFuzzer, Fuzzer};
```

And create the fuzzer:

```rust
let mut fuzzer = StdFuzzer::new(scheduler, counters_feedback, objective);
```
