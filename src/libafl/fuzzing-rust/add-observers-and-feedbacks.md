# Add Observers and Feedbacks

We've [already discussed](#coverage-sanitizer) how we are instrumenting our target to
keep track of coverage information, but we need a way to incorporate the information
we get from our 8-bit edge hit counters as feedback to our fuzzer.

LibAFL has two concepts that allow us to do this:
