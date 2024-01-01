# Start The Fuzz Loop

The *very* final step, we need to actually start our fuzzer! This will start our
fuzzer and it will run until it gets an exit condition. For this simple fuzzer, because
we are running our harness in process, we will exit when the target crashes for the
first time. Typically, we would continue to fuzz.

```rust
fuzzer
    .fuzz_loop(&mut stages, &mut executor, &mut state, &mut mgr)
    .expect("Error in the fuzzing loop");
```
