# State

Now that we have created all the parts of the fuzzer that make up the
[`State`](https://docs.rs/libafl/latest/libafl/state/trait.State.html), we can create
one. The [`State`](https://docs.rs/libafl/latest/libafl/state/trait.State.html) trait
only has one default implementation. The state tracks all the information in the fuzzing
campaign including metadata like executions, the corpus, and more.

Add the `use` declarations we need:

```rust
use libafl::state::StdState;
```

And we'll create a
[`StdState`](https://docs.rs/libafl/latest/libafl/state/struct.StdState.html) with
our corpus, random provider, feedback, and objective:


```rust
let mut state = StdState::new(
    rand,
    corpus,
    solutions,
    &mut counters_feedback,
    &mut objective,
)
.expect("Failed to create state");
```
