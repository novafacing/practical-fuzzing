# Choosing Good Fuzz Targets

The first thing we need to do when we're building a fuzzer is to identify what we want
to fuzz. What makes a good fuzz target is complicated, and there are many factors to
consider. In general, though, here are a few guidelines to keep an eye out for:

1. Code that consumes untrusted data: any code that reads data from the outside world,
   especially if the data comes from an untrusted source (i.e. comes from a user of your
   code) is a great place to use fuzzing. Fuzzing can help find security issues with how
   your code processes this data, as well as functionality and logic issues, especially
   if your project includes assertions. Code patterns that commonly fit this criteria
   include:
   - Parsers, like those for web forms or API requests in JSON or similar formats
   - OS or Library API functionality, for example when adding a system call to Linux or
   exporting any function from a library
   - Command line tools that take input
   - Update utilities that download update files from the internet
2. Code that is security critical or runs with elevated privilege: if your code runs as
   root or below (or an equivalent), it should be fuzzed. Even a single off-by-one error
   in a `memcpy` call is a critical security vulnerability in this scenario, as it
   gives way to privilege escalation. If your code is in this category, *all* of it
   should be fuzzed, not just external interfaces. We'll discuss later on some methods
   to speed up harnessing in this type of scenario.

Some additional resources for identifying good fuzz targets:

- [Google Fuzzing Guide](https://github.com/google/fuzzing/blob/master/docs/good-fuzz-target.md)

