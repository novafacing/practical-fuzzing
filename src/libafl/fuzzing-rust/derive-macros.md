# Derive Macros

*Derive macros* allow us to add an *attribute* to a struct 
that automatically adds some functionality for us. In this case, deriving `Parser`
automatically adds the `parse` static method to the `Args` struct. Calling it will
automatically parse command line arguments for us (including some default extras, like
`-h` for help) and give us an initialized `Args` struct we can use to grab the two paths
we have as arguments. We give extra info to the `derive` by adding *attributes* to the 
fields of the struct as well, so `clap` knows we want to have short and long argument
flags for those fields, instead of them being positional arguments.
