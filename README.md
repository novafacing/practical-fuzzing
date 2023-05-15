# LibAFL Training

So, you want to learn how to use
[LibAFL](https://github.com/AFLplusplus/LibAFL)! Welcome to the future of
fuzzing. You can read all about [LibAFL](https://github.com/AFLplusplus/LibAFL)
and the features it offers on their [github](https://github.com/AFLplusplus/LibAFL)
page, but the highlights are:

- Fast: LibAFL lets you create the fastest fuzzers possible, with minimal overhead.
- Rust: Take advantage of the Rust language and ecosystem, including package management
  and robust FFI.
- Scaling: LibAFL scales across cores and machines by default.
- Any platform: LibAFL runs on Windows, Linux, Android, MacOS, and embedded environments.
- *Any* target: LibAFL can fuzz targets on any platform it runs on, and any other
  platform by creating remote harnesses. Fuzz FPGAs, UEFI firmware, Web Apps, and more,
  with or without source code.
- Customizable: LibAFL gives you robust building blocks to assemble powerful fuzzers,
  but every component can be swapped out and customized for your use case or for maximum
  performance.

# Training Overview

This training will teach you beginner through advanced use of the LibAFL library to
build your own fuzzers. Unlike other LibAFL tutorials, it doesn't assume you are a Rust
expert, or even that you have any Rust experience. It *does* however, assume that you
are an experienced C/C++ programmer, and will explain Rust concepts throughout in terms
of their C/C++ equivalents.

# Training Program

## Chapter 1: Learn the Ropes

1. [Set up your development environment](./docs/Setup.md)
2. [Learn the basics of the Rust ecosystem](./docs/RustBasics.md)
3. [Compile and run your first fuzzer](./docs/RunFirstFuzzer.md)
4. [Write your first fuzz target](./docs/WriteFirstTarget.md)
5. [Learn the parts of a LibAFL fuzzer](./docs/LearnLibAFLParts.md)
6. [Write a fuzzer for your first fuzz target](./docs/WriteFirstFuzzer.md)

## Chapter 2: Fuzzing Userspace C/C++

1. [Learn to build libraries with `libafl_cc` and `libafl_cxx`](./docs/BuildLibraries.md)
2. [Write a harness for `hyperscan`](./docs/HarnessHyperscan.md)
3. [Learn to build binaries with `libafl_cc` and `libafl_cxx`](./docs/BuildBinaries.md)
4. [Write a harness for `jq`](./docs/HarnessJq.md)

## Chapter 3: Fuzzing Other Languages In Userspace

1. [Fuzzing Rust](./docs/FuzzingRust.md)
2. [Fuzzing Rust for Windows](./docs/FuzzingRustWindows.md)
3. [Fuzzing C/C++ for Windows (Clang)](./docs/FuzzingCCppWindowsClang.md)
4. [Fuzzing C/C++ for Windows (MSVC)](./docs/FuzzingCCppWindowsMSVC.md)
5. [Fuzzing C#](./docs/FuzzingCSharp.md)
6. [Fuzzing C# for Windows](./docs/FuzzingCSharpWindows.md)
7. [Fuzzing Python](./docs/FuzzingPython.md)
8. [Fuzzing Java, Kotlin, and Scala](./docs/FuzzingJavaKotlinScala.md)
9. [Fuzzing Go](./docs/FuzzingGo.md)

## Chapter 4: Extending Fuzzers

1. [Write a custom Feedback and Observer](./docs/WriteCustomFeedbackObserver.md)
2. [Write a structured data Mutator](./docs/WriteMutator.md)
3. [Write a corpus Scheduler](./docs/WriteScheduler.md)
4. [Write a custom Stage](./docs/WriteStage.md)
5. [Use QEMU mode](./docs/UsingQEMUMode.md)
6. [Use FRIDA mode](./docs/UsingFRIDAMode.md)
7. [Add concolic execution](./docs/AddConcolicExecution.md)
8. [Use NYX mode](./docs/UsingNyxMode.md)

## Chapter 5: Advanced Usage and Writing Research Fuzzers

1. [The CONFUSE fuzzer](./docs/CONFUSE.md)
2. [The kAFL fuzzer](./docs/kAFL.md)
3. [Implementing new methods](./docs/ImplementingNewMethods.md)
4. [Targeting complex systems](./docs/TargetingComplexSystems.md)

# Useful Resources

No training can be completely exhaustive, so you may find these resources useful
throughout in order to understand various Fuzzing, Rust, and LibAFL concepts.

## Rust

- [Learn X in Y Minutes: Rust](https://learnxinyminutes.com/docs/rust/)
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [The Rustonomicon: Unsafe Rust](https://doc.rust-lang.org/nomicon/)
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/index.html)
- [Cargo (Rust Package Manager) Book](https://doc.rust-lang.org/cargo/index.html)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings: Rust Exercises](https://github.com/rust-lang/rustlings/)

## Fuzzing

<!-- TODO: Add links to fuzzing overview and fuzzing training videos -->

## LibAFL

- [LibAFL Repository](https://github.com/AFLplusplus/LibAFL)
- [LibAFL Documentation](https://docs.rs/libafl/latest/libafl/)
- [LibAFL Book (Incomplete)](https://aflplus.plus/libafl-book)
- [Fuzzing101 Blog Posts](https://epi052.gitlab.io/notes-to-self/blog/2021-11-01-fuzzing-101-with-libafl/)
- [LibAFL Paper](https://www.s3.eurecom.fr/docs/ccs22_fioraldi.pdf)
