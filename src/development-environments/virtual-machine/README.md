# Virtual Machines

Many fuzzing targets can run and be fuzzed entirely in user-space, and many fuzzers
require no special kernel modifications or particular operating systems to run them.
When this is *not* the case, it is usually most convenient to use a Virtual Machine, for
example when learning to fuzz Kernel Modules/Drivers (on both Windows and Linux), or
when particular resources are needed. We provide guides for assembling these
development or fuzzing environments that are used throughout this book.

- [Windows Kernel Development](windows-kernel/README.md)