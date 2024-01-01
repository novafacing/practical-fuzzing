# Declare Functions From Target

Because we are linking with our target statically, we need to declare the functions
we want to call from it as `extern`. This has the same meaning as in C/C++, and informs
the compiler that we will link with these symbols, but we are not defining them.

Add the following block below the allocator definition.

```rust
extern "Rust" {
    fn decode(encoded_input: &[u8]) -> Vec<u8>;
    fn counters_maps_observer(name: &'static str) -> CountersMultiMapObserver<false>;
}
```
