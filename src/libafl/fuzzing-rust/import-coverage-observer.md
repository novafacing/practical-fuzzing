# Import Coverage Observer

We'll discuss the coverage observer in greater detail later, but you can `use` it from
the `libafl_targets` dependency like so:


```rust
use libafl_targets::CountersMultiMapObserver;
```

This is necessary so we have the type of the observer available to us.
