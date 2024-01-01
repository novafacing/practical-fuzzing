# Load the Input Corpus

Before we start fuzzing, we need to load our input corpus. We can load as many
corpus entries as we want, and we will load them from our corpus directory argument.

Each initial input will be executed to make sure there is some interesting feedback.

This is a good litmus test to ensure your fuzzer works as intended -- if the fuzzer
fails when loading inputs, there is likely something wrong with your observer(s) or
feedback(s).

```rust
state
    .load_initial_inputs(&mut fuzzer, &mut executor, &mut mgr, &[args.corpus])
    .expect("Failed to generate the initial corpus");
```
