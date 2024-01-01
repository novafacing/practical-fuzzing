# Observers

An *Observer* in LibAFL implements the
[`Observer`](https://docs.rs/libafl/latest/libafl/observers/trait.Observer.html) trait. An
[`Observer`](https://docs.rs/libafl/latest/libafl/observers/trait.Observer.html) is very
generic and does essentially what it sounds like: observes some state.
[`Observer`](https://docs.rs/libafl/latest/libafl/observers/trait.Observer.html)s
optionally implement methods that are called on events that occur each fuzzing iteration
like `pre_exec`, `post_exec`, and so forth.

The most familiar [`Observer`](https://docs.rs/libafl/latest/libafl/observers/trait.Observer.html)
type is the [`HitcountsMapObserver`](https://docs.rs/libafl/latest/libafl/observers/map/struct.HitcountsMapObserver.html) that observes a region of memory (a bitmap) whose entries are
incremented each time an edge or block is hit during a target execution. This is the
model AFL++ uses by default.

[`Observer`](https://docs.rs/libafl/latest/libafl/observers/trait.Observer.html)s can
also observe other observes, that is, they can be nested. For example, a [`HitcountsMapObserver`](https://docs.rs/libafl/latest/libafl/observers/map/struct.HitcountsMapObserver.html) observes a
[`MapObserver`](https://docs.rs/libafl/latest/libafl/observers/map/trait.MapObserver.html)
and adds post-processing of hitcounts instead of the raw map data.

We can use one of the many provided implementations, or we can define our own, like with
everything in LibAFL.
