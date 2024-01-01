# Mutator

The [`Mutator`](https://docs.rs/libafl/latest/libafl/mutators/trait.Mutator.html) is
similar to the scheduler in terms of how complex it can become if you want it to. In our
case, we'll again opt for the simplest option, the
[`StdScheduledMutator`](https://docs.rs/libafl/latest/libafl/mutators/scheduled/struct.StdScheduledMutator.html).

Add a `use` for the mutator and the well known *havoc* set of mutation strategies which
includes common operations like bit and byte flips, pastes, add and subtract, and more,
which we'll use in our mutator.

```rust
use libafl::prelude::{StdScheduledMutator, havoc_mutations};
```

And create the mutator. 

```rust
let mutator = StdScheduledMutator::new(havoc_mutations());
```
