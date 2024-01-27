# Default

This test-suite is an integration test that 
ensures some basic behavior and look.

## What does it tests?

* Visual
    * Once defined this must not change due to bugs
    * Passing and Failing messages
        * Checks the visual consistence
    * ANSI colors
        * Must check the ANSI color
* Finder recursion
    * Must recursively find files inside the `test` folder
* Finder selection
    * Must select files with `test` prefix
    * Must select `.art` files
* Statistics
    * Must check the total
    * Must check how many tests suceeded
    * Must check how many tests failed

## What doesn't it test?

* Return codes
    * Unhandled exceptions
    * Importing errors
    * Segfaults