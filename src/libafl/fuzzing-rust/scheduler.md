# Scheduler

Scheduling test cases ranges from very easy (test all cases in order as they are
generated, or randomly) to very complex (minimization, environmental friendliness, coverage
accounting). LibAFL of course also allows you to define new corpus scheduling methods
as well, but for our purposes a simple queue is more than sufficient.

Add the `use` declaration for the [`QueueScheduler`](https://docs.rs/libafl/latest/libafl/schedulers/queue/struct.QueueScheduler.html):

```rust
use libafl::prelude::QueueScheduler;
```

And create one:

```rust
let scheduler = QueueScheduler::new();
```

