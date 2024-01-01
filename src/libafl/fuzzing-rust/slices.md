# Slices

We just need one more concept before we're ready to move on with our fuzz target:
slices!

A slice is just a *reference* to a sequence of items with the same type. They can be
created literally, as with this creation of a slice referencing the sequence of two
strings:

```rust
fn main() {
    let _x: &[String] = &[
        "Hello, World!".to_string(),
        "I'm here!".to_string()
    ];
}
```

<center>
<a href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn+main%28%29+%7B%0A++++let+_x%3A+%26%5BString%5D+%3D+%26%5B%0A++++++++%22Hello%2C+World%21%22.to_string%28%29%2C%0A++++++++%22I%27m+here%21%22.to_string%28%29%0A++++%5D%3B%0A%7D">Playground</a>
</center>
<br>

They can also be created by referencing another data structure. For example a slice of a
`Vec` (equivalent to a C++ `std::vector`) is a reference to a sequence of its entries.
We can also take a slice of a `String`'s characters as a sequence of bytes using
`.as_bytes()`. Slices are very flexible, and many methods of various types yield slices
of different underlying element types (such as `as_bytes()`).

```rust
fn main() {
    let v: Vec<String> = vec![
        "Hello, World!".to_string(),
        "I'm here!".to_string()
    ];
    let _x: &[String] = &v;
    let u: String = "Hello, World!".to_string();
    let _y: &[u8] = u.as_bytes();
}
```

<center>
<a href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn+main%28%29+%7B%0A++++let+v%3A+Vec%3CString%3E+%3D+vec%21%5B%0A++++++++%22Hello%2C+World%21%22.to_string%28%29%2C%0A++++++++%22I%27m+here%21%22.to_string%28%29%0A++++%5D%3B%0A++++let+_x%3A+%26%5BString%5D+%3D+%26v%3B%0A++++let+u%3A+String+%3D+%22Hello%2C+World%21%22.to_string%28%29%3B%0A++++let+_y%3A+%26%5Bu8%5D+%3D+u.as_bytes%28%29%3B%0A%7D">Playground</a>
</center>
<br>
