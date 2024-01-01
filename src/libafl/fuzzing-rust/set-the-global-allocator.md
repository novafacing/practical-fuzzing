# Set The Global Allocator

Rust allows us to choose any allocator we want as the global allocator that will be used
for all our allocations. We are using the
[mimalloc](https://github.com/microsoft/mimalloc) allocator. We can set it as the
global allocator by adding a couple lines to our fuzzer's `main.rs` above the `main`
function.

```rust
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
```

This lets us use a different allocator in our fuzzer than is being called by our target,
so when the target malloc state gets corrupted we won't crash the fuzzer.
