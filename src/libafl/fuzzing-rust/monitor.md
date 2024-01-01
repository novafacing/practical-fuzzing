# Monitor

A [`Monitor`](https://docs.rs/libafl/latest/libafl/monitors/trait.Monitor.html) tracks
the state of the fuzzing campaign and displays information and statistics to the user.
There are many options for [`Monitor`](https://docs.rs/libafl/latest/libafl/monitors/trait.Monitor.html)s
from full-featured TUIs to basic loggers. We will just print out every message we see
to the terminal, the simplest possible logger.

Add the `use` declaration:

```rust
use libafl::prelude::SimpleMonitor;
```

And create the monitor, wrapping a closure that prints its argument:

```rust
let mon = SimpleMonitor::new(|s| println!("{}", s));
```

