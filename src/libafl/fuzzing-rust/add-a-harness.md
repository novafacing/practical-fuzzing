# Add a Harness

The first part of our fuzzer we'll create is our harness.  A *harness* is the function
our fuzzer will actually call directly for each test case it tests. We'll use a
*closure* instead of defining another function, because our harness will be very short.