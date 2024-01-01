# Coverage Sanitizer

Before we use it, we need a bit of background on what a *coverage sanitizer* actually is,
as well as what sanitizer interface we will use for this fuzzer.

LLVM and MSVC provide a feature called
[SanitizerCoverage](https://clang.llvm.org/docs/SanitizerCoverage.html). Code coverage
is a relatively simple concept, and it is explained well by the documentation. Our goal
is simply to measure *how much* of the code we care about is executed as well as *which
parts* of the code we care about are executed for each fuzzing run.

`SanitizerCoverage` is implemented at the LLVM level of compilation, and has several
optional implementations:

- PC Trace: a function is called on each control flow edge in the program.
- Guards: a function is called on each control flow edge in the program that is only
  triggered if a guard value is true. Same as PC trace with guards added.
- Counters: a per-edge counter variable is incremented on each traversal of the edge
- PC Table: Adds a table describing whether each block starting at `pc` is a function
  entry block or not

The exact arguments change over time with new LLVM releases, but [the LLVM
Doxygen](https://llvm.org/doxygen/SanitizerCoverage_8cpp.html) describes all of them.

For this fuzzer, we will use the following options for `SanitizerCoverage`:

- `-sanitizer-coverage-level=3`: Level 3 is all blocks and critical edges.
- `-sanitizer-coverage-inline-8bit-counters`: Use counters to track how many times each
  edge is hit.
- `-sanitizer-coverage-prune-blocks=0`: Don't remove coverage info from any blocks.
- `passes=sancov-module`: Tells LLVM to run the `SanitizerCoverage` module.
