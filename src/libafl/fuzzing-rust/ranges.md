# Ranges

The `.get()` call uses a *range* (`0..2`) from `0` to `2`, inclusive on the low and
exclusive on the high end of the range: 

```rust
if let Some(&[first, second]) = rest.get(0..2) {
```

Ranges are first-class objects in Rust and work in a similar way as in Python (although
with `..` instead of `:`). The [String
Slices](https://doc.rust-lang.org/book/ch04-03-slices.html?highlight=range#string-slices)
section goes into depth on slice syntax. 

The rest of the code parses the escape hex codes and takes an appropriate action based
on the values, writing to the output data and incrementing the pointer we are writing
through as needed. Finally, we construct a vector from the pointer, decoded size, and a
*capacity* (which we just provide the size as) and return it. Note that unlike earlier,
the `unsafe` code we are running here truly is *unsafe*. Because we don't calculate our
buffer size correctly, these writes can go out of bounds of the allocated memory,
causing heap corruption.
