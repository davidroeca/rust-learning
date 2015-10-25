## Simple Calculator Implemented in Rust

This project is still very much a work in progress, though I'm going to move away from it for a bit. Currently the interactions are buggy and only work if they're wrapped in parentheses. Likewise, +, -, /, and * are the only valid operators. If/when I come back to this, below is the outline of future development.

## Next Steps

1. Parse expressions and wrap parentheses to allow for PEMDAS instead of producing an error.
  * E in PEMDAS--add a ^ exponent operator.
  * Allow for assignment in expressions (like C)
* __Modularize__ code more.
  * Currently the code is somewhat large and bloated.
  * Figure out a better design for the nested functions in parse_expression().
* Potentially fix string instantiation in the tokenization phase by using the correct slicing method.
* Make the code more object-oriented, to group functions in more effective namespaces and intuitive structures.
