# Event Manager

The [`EventManager`](https://docs.rs/libafl/latest/libafl/events/trait.EventManager.html)
handles events that occur during fuzzing and takes action as needed. For a single-core
local fuzzer, this handling is simple, it primarily passes events to the `Monitor` to
be displayed, but there are complex event managers that handle synchronization across
machines, restart the fuzzer on specific events, and more.

We'll use the [`SimpleEventmanager`](https://docs.rs/libafl/latest/libafl/events/simple/struct.SimpleEventManager.html). Add the `use` declaration:


```rust
use libafl::prelude::SimpleEventManager;
```

And create the manager:

```rust
let mut mgr = SimpleEventManager::new(mon);
```

