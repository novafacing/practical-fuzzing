# Decode Encoded Input

Now, we'll do our actual decoding and write the decoded data into our pointer.

```rust
use std::{
    alloc::{alloc, Layout},
    str::from_utf8_unchecked,
};


pub fn decode(mut encoded_input: &[u8]) -> Vec<u8> {
    /* ... Code from above ... */
    loop {
        let mut parts = encoded_input.splitn(2, |&c| c == b'%');
        let not_escaped_part = parts.next().expect("Did not get any unescaped data");
        let rest = parts.next();
```

The first line here, `loop {` is a *loop statement*, which will run the block inside the
loop forever, until either a `break` or `return` happens.

Each time we run our loop, we split our encoded input on the first '%' character,
splitting it into at most 2 parts (hence `splitn`). The first part is any data at the
beginning of the encoded input before the first '%', the second part is the rest of the
encoded input after the first '%', if there is any. `parts` is an *iterator* over
sub-slices of type `&[u8]`.

`Iterator`s over an `Item` type (in this case, `&[u8]`) are types that have a `next()`
function to return the next `Item` if there is one, and `None` if there are no more
items (using an `Option` type, with the same semantics as Python's `Optional[]` and
C++'s `std::option`). Rust's iterators are extremely similar to Python's, and somewhat
similar to C++'s in theory (they both allow you to iterate over entries of some
collection), although not in implementation.


```rust
        if rest.is_none() && decoded == decoded_ptr {
            return encoded_input.to_vec();
        } else {
```

Next, we check to see if we have anything in `rest`. If not, the `%` was at the end of
the input, or there was no '%'. If this is the case and we haven't written any decoded
data to our output, we can return the encoded input as the result, converting the slice
to a `Vec` (which performs an allocation).

```rust
            for not_escaped_byte in not_escaped_part {
                unsafe { *decoded_ptr = *not_escaped_byte };
                unsafe { decoded_ptr = decoded_ptr.add(1) };
            }
```

If we didn't return early, we always write our "not escaped" data to our output. This
operation is exactly equivalent to `*ptr = b; ptr++;` in C, we simply need to wrap this
operation in `unsafe`, because we are dereferencing a raw pointer. `++` and `+=` are
also not defined on raw pointers, but we can use the `.add()` method, which is

```rust
            if let Some(rest) = rest {
                if let Some(&[first, second]) = rest.get(0..2) {
                    if let Ok(first_val) =
                        u8::from_str_radix(unsafe { from_utf8_unchecked(&[first]) }, 16)
                    {
                        if let Ok(second_val) =
                            u8::from_str_radix(unsafe { from_utf8_unchecked(&[second]) }, 16)
                        {
                            unsafe { *decoded_ptr = (first_val << 4) | second_val };
                            unsafe { decoded_ptr = decoded_ptr.add(1) };
                            encoded_input = &rest[2..];
                        } else {
                            unsafe { *decoded_ptr = b'%' };
                            unsafe { decoded_ptr = decoded_ptr.add(1) };
                            unsafe { *decoded_ptr = first };
                            unsafe { decoded_ptr = decoded_ptr.add(1) };
                            encoded_input = &rest[1..];
                        }
                    } else {
                        unsafe { *decoded_ptr = b'%' };
                        unsafe { decoded_ptr = decoded_ptr.add(1) };
                        encoded_input = rest;
                    }
                } else {
                    unsafe { *decoded_ptr = b'%' };
                    unsafe { decoded_ptr = decoded_ptr.add(1) };

                    for rest_byte in rest {
                        unsafe { *decoded_ptr = *rest_byte };
                        unsafe { decoded_ptr = decoded_ptr.add(1) };
                    }

                    break;
                }
            } else {
                break;
            }
        }
    }

    unsafe { Vec::from_raw_parts(decoded, decoded_len, decoded_len) }
}
```
 
The rest of the code is somewhat difficult to break up coherently. There are only a
few important new concepts, however.
