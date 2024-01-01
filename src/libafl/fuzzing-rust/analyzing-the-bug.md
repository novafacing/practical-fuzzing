# Analyzing the Bug

Our unit tests pass! Now lets check out what happens if we uncomment the third function.

```sh
$ cargo test
   Compiling first-targetv0.1.0 (/workspaces/documentation.security.fuzzing.libafl/first-target)
    Finished test [unoptimized + debuginfo] target(s) in 0.27s
     Running unittests src/lib.rs (target/debug/deps/first_target-49e422aa7de12c55)

running 3 tests
malloc(): corrupted top size
error: test failed, to rerun pass `--lib`

Caused by:
  process didn't exit successfully: `/workspaces/documentation.security.fuzzing.libafl/first-target/target/debug/deps/first_target-49e422aa7de12c55` (signal: 6, SIGABRT: process abort signal)
```


The program crashes with an error in `libc` `malloc`. We've corrupted the top chunk
size, so malloc crashes. We can investigate exactly where this error occurs in the
malloc source code by searching the error message, and we find out it happens
[here](https://github.com/bminor/glibc/blob/7c32cb7dd88cf100b0b412163896e30aa2ee671a/malloc/malloc.c#L4359).
What's happening is that we only have one allocation in our program, so the heap looks
like the image below. Recall the heap grows from low to high addresses (left to right
below).

<center>
<img src="./diagrams/WriteFirstFuzzerTopChunk.svg" />
</center>
<br>

The "top chunk" is a special chunk that contains all the memory that hasn't been
allocated yet, and is also known as the "wilderness". When we write out of bounds of the
memory we allocated for `decoded`, we overwrite the `mchunk_size` field of the metadata
of the top chunk. This can lead to an attack known as [House of
Force](https://github.com/shellphish/how2heap/blob/master/glibc_2.27/house_of_force.c).
This specific attack isn't important for our fuzzing, but it's a good case study of how
dangerous an error as simple as naive size calculation can be. Not to mention, we end up
incorrectly handling trailing `%` characters!

If you don't see an error on your system, that's ok. It's somewhat dependent on what
`libc` version is in use (if you are on Linux), and what size of overflow we end up
performing. Even if this function doesn't crash on your system, our fuzzer will be able
to crash it with some input it will come up with. Go ahead and re-comment that
test function.
